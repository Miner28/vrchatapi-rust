/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Verify2FaEmailCodeResult {
    #[serde(rename = "verified")]
    pub verified: bool,
}

impl Verify2FaEmailCodeResult {
    pub fn new(verified: bool) -> Verify2FaEmailCodeResult {
        Verify2FaEmailCodeResult {
            verified,
        }
    }
}

