use std::borrow::Borrow;
use std::iter::Filter;
use std::ptr::null;
use std::result::Iter;
use std::str::Chars;
use opensky_api::errors::Error;

pub enum PlaneType {
    Commercial,
    Trainer,
    Cargo,
    Business,
    Unknown,
}

impl PlaneType {
    pub fn to_str(self) -> &'static str {
        match self {
            PlaneType::Commercial => "Commercial",
            PlaneType::Cargo => "Cargo",
            PlaneType::Trainer => "Trainer",
            PlaneType::Business => "Business",
            PlaneType::Unknown => "Unknown",
        }
    }
}


struct Plane {
    longitude: f32,
    latitude: f32,
    altitude: f32,
    track: f32,
    speed: f32,
    airline: BasicAirline,
    plane_type: PlaneType,
    callsign: String,
}

impl Plane {
    fn new(longitude: f32, latitude: f32, altitude: f32, track: f32, speed: f32, airline: BasicAirline, plane_type: PlaneType, callsign: String) -> Self {
        Plane {
            longitude,
            latitude,
            altitude,
            track,
            speed,
            airline,
            plane_type,
            callsign,
        }
    }

    fn to_plane(self) -> Plane {
        let ret = Plane::new(self.longitude, self.latitude, self.altitude, self.track, self.speed, self.airline, self.plane_type, self.callsign);
        return ret
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Airline {
    Basic(BasicAirline),
    Dynamic(DynamicAirline),
    Unknown,
}

#[derive(Clone, PartialEq, Eq)]
pub struct DynamicAirline {
    pub callsign: String,
    pub name: String,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BasicAirline {
    American,
    Spirit,
    Southwest,
    United,
    Delta,
    Unknown,
    Other,
}


impl BasicAirline {
    pub fn to_str(&self) -> &str {
        match self {
            BasicAirline::American => "American Airlines",
            BasicAirline::Spirit => "Spirit Airlines",
            BasicAirline::Southwest => "Southwest Airlines",
            BasicAirline::United => "United Airlines",
            BasicAirline::Delta => "Delta Airlines",
            BasicAirline::Unknown => "Unknown",
            BasicAirline::Other => "other",
        }
    }
}

impl Airline {
    pub fn to_str(&self) -> &str {
        match self {
            Airline::Basic(basic) => basic.to_str(),
            Airline::Unknown => "Unknown",
            Airline::Dynamic(s) => s.name.as_str(),
        }
    }
}

impl From<BasicAirline> for Airline {
    fn from(basic: BasicAirline) -> Self {
        Airline::Basic(basic)
    }
}

struct PlaneBody {
    planes: Vec<Plane>,
    airline: Airline,
    plane_type: PlaneType,
}

impl PlaneBody {
    pub fn new(planes: Vec<Plane>, airline: Airline, plane_type: PlaneType) -> Self {
        PlaneBody {
            planes,
            airline,
            plane_type,
        }
    }
    pub fn empty_commercial(airline: Airline) -> Self {
        PlaneBody {
            planes: Vec::new(),
            airline,
            plane_type: PlaneType::Commercial,
        }
    }
}

#[tokio::main]
async fn main() {
    let planes = request_plane().await;
    let _planes: Vec<Plane> = planes.unwrap();
    let lookup = lookup_plane(_planes, "UAL1099".to_string()).await;
    println!("Tracked Aircraft: {:?}, Latitude: {:?}, Longitude: {:?}, Track: {:?}, Altitude: {:?}, Speed: {:?}",
             lookup.callsign, lookup.latitude, lookup.longitude, lookup.track, lookup.altitude, lookup.speed);
}

async fn request_plane() ->Result<Vec<Plane>, Error> {
    let open_sky = opensky_api::OpenSkyApi::new();
    let state_request = open_sky.get_states();

    let mut list_of_planes: Vec<Plane> = Vec::new();

    let open_sky = state_request.send().await?;

    for state in open_sky.states {
        let call = state.callsign.as_ref().unwrap();
        let lat = state.latitude;
        let long = state.longitude;
        let track = state.true_track;
        let alt = state.baro_altitude;
        let knots = state.velocity;
        let _airline = &call.to_string();
        let mut delta_airlines: PlaneBody = PlaneBody::empty_commercial(BasicAirline::Delta.into());
        let mut spirit_airlines: PlaneBody = PlaneBody::empty_commercial(BasicAirline::Spirit.into());
        let mut american_airlines: PlaneBody = PlaneBody::empty_commercial(BasicAirline::American.into());
        let mut southwest_airlines: PlaneBody = PlaneBody::empty_commercial(BasicAirline::Southwest.into());
        let mut united_airlines: PlaneBody = PlaneBody::empty_commercial(BasicAirline::United.into());
        let mut maybe_airline = None;
        let mut callsign_header: &str = "";

        if call == "" { callsign_header = ""; }
        else { callsign_header = &call[0..3]; }

        match callsign_header {
            "NKS" => maybe_airline = Some(BasicAirline::Spirit.into()),
            "AAL" => maybe_airline = Some(BasicAirline::American.into()),
            "SWA" => maybe_airline = Some(BasicAirline::Southwest.into()),
            "UAL" => maybe_airline = Some(BasicAirline::United.into()),
            "DAL" => maybe_airline = Some(BasicAirline::Delta.into()),
            _ => maybe_airline = Some(BasicAirline::Other.into()),
        }


        let plane = Plane::new(
            long.unwrap_or_default(),
            lat.unwrap_or_default(),
            alt.unwrap_or_default(),
            track.unwrap_or_default(),
            knots.unwrap_or_default(),
            maybe_airline.clone().unwrap_or(BasicAirline::Other),
            PlaneType::Commercial,
            call.clone()
        );

        let _plane = Plane::new(
            long.unwrap_or_default(),
            lat.unwrap_or_default(),
            alt.unwrap_or_default(),
            track.unwrap_or_default(),
            knots.unwrap_or_default(),
            maybe_airline.clone().unwrap_or(BasicAirline::Other),
            PlaneType::Commercial,
            call.clone()
        );

        list_of_planes.push(_plane);

        //let plane_body = PlaneBody::new(plane, airline, PlaneType::Unknown);

        println!("Aircraft info: Callsign: {:?}, Company: {:?}, Latitude: {:?}, Longitude: {:?}, Altitude: {:?}, Track: {:?}, Speed: {:?}",
                 call, &plane.airline.to_str(), &plane.latitude, &plane.longitude, &plane.altitude, &plane.track, &plane.speed);
    }

    //lookup_plane(list_of_planes, "DAL");

    Ok(list_of_planes)
}

async fn lookup_plane(planes: Vec<Plane>, callsign: String) -> Plane {

    let return_plane =
        Plane::new(0.0,
                   0.0,
                   0.0,
                   0.0,
                   0.0,
                   BasicAirline::Unknown,
                   PlaneType::Unknown,
                   "null".to_string());

    //callsign.push(' ');

    for _plane in planes {
    }

    return return_plane
}
