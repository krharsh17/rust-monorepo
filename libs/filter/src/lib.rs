use censor::*;

pub fn filter_ableism(text: String) -> String {

    let censor = Censor::Standard + "lame" + "dumb" + "retarded" + "blind" + "deaf";

    return censor.censor(&text);
}

pub fn filter_violence(text: String) -> String {
    let censor = Censor::Standard + "attack" + "dead" + "murder" + "kill";

    return censor.censor(&text);
}