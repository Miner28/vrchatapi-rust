/*
 * VRChat API Documentation
 *
 * ![VRChat API Banner](https://vrchatapi.github.io/assets/img/api_banner_1500x400.png)  # Welcome to the VRChat API  Before we begin, we would like to state this is a **COMMUNITY DRIVEN PROJECT**. This means that everything you read on here was written by the community itself, and is **not** officially supported by VRChat. The documentation and additional libraries SHALL ONLY be used for applications interacting with VRChat's API, in accordance with their [Terms of Service](https://github.com/VRChatAPI), and MUST NOT be used for modifying the client, \"avatar ripping\", or other illegal activities.  This documentation is provided \"AS IS\", and any action you take towards VRChat is completely your own responsibility. Malicious usage or spamming the API may result in account termination. Certain parts of the API are also more sensitive than others, for example moderation, so please read tread extra carefully and read the warnings when present.  ![Tupper Policy on API](https://i.imgur.com/yLlW7Ok.png)  Finally, use of the API using applications other than the approved methods (website, VRChat application, Unity SDK) is not officially supported. VRChat provides no guarantee for external applications using the API. Access to API endpoints may break **at any time, without notice**. We will make a best effort in keeping this documentation and associated language libraries up to date, but things might be outdated or missing. If you find that something is no longer valid, please contact us on Discord or [create an issue](https://github.com/vrchatapi/specification/issues) and tell us so we can fix it.  # Getting Started  The VRChat API can be used to programatically retrive or update information regarding your profile, friends, avatars, worlds and more. The API consists of two parts, \"Photon\" which is only used in-game, and the \"Web API\" which is used by both the game and the website. This documentation focuses only on the Web API.  The API is designed around the REST ideology, providing semi-simple and usually predictable URIs to access and modify objects. Requests support standard HTTP methods like GET, PUT, POST, and DELETE and standard status codes. Response bodies are always UTF-8 encoded JSON objects, unless explicitly documented otherwise.  <div class=\"callout callout-error\">   <strong>🛑 Warning! Do not touch Photon!</strong><br>   Photon is only used by the in-game client and should <b>not</b> be touched. Doing so may result in permanent account termination. </div>  <div class=\"callout callout-info\">   <strong>ℹ️ API Key and Authentication</strong><br>   The API Key has always been the same, and is currently <code>JlE5Jldo5Jibnk5O5hTx6XVqsJu4WJ26</code>.   Read <a href=\"#tag--authentication\">Authentication</a> for how to log in. </div>  # Using the API  For simply exploring what the API can do it is strongly recommended to download [Insomnia](https://insomnia.rest/download), a free and open-source API client that's great for sending requests to the API in an orderly fashion. Insomnia allows you to send data in the format that's required for VRChat's API. It is also possible to try out the API in your browser, by first logging in at [vrchat.com/home](https://vrchat.com/home/) and then going to [vrchat.com/api/1/auth/user](https://vrchat.com/api/1/auth/user), but the information will be much harder to work with.  For more permanent operation such as software development it is instead recommended to use one of the existing language SDKs. This community project maintain API libraries in several languages, which allows you to interact with the API with simple function calls rather than having to implement the HTTP protocol yourself. Most of these libraries are automatically generated from the API specification, sometimes with additional helpful wrapper code to make usage easier. This allows them to be almost automatically updated and expanded upon as soon as a new feature is introduced in the specification itself. The libraries can be found on [GitHub](https://github.com/vrchatapi) or following:  * [NodeJS (JavaScript)](https://www.npmjs.com/package/vrchat) * [Dart](https://pub.dev/packages/vrchat_dart) * [Rust](https://crates.io/crates/vrchatapi) * [C#](github.com/vrchatapi/vrchatapi-csharp) * [Python](https://github.com/vrchatapi/VRChatPython)  # Pagination  Most endpoints enforce pagination, meaning they will only return 10 entries by default, and never more than 100.<br> Using both the limit and offset parameters allows you to easily paginate through a large number of objects.  | Query Parameter | Type | Description | | ----------|--|------- | | `limit` | integer  | The number of objects to return. This value often defaults to 10. Highest limit is always 100.| | `offset` | integer  | A zero-based offset from the default object sorting.|  If a request returns fewer objects than the `limit` parameter, there are no more items available to return.  # Contribution  Do you want to get involved in the documentation effort? Do you want to help improve one of the language API libraries? This project is an [OPEN Open Source Project](https://openopensource.org)! This means that individuals making significant and valuable contributions are given commit-access to the project. It also means we are very open and welcoming of new people making contributions, unlike some more guarded open source projects.  [![Discord](https://img.shields.io/static/v1?label=vrchatapi&message=discord&color=blueviolet&style=for-the-badge)](https://discord.gg/qjZE9C9fkB)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Instance {
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "canRequestInvite")]
    pub can_request_invite: bool,
    #[serde(rename = "capacity")]
    pub capacity: f32,
    #[serde(rename = "clientNumber")]
    pub client_number: String,
    #[serde(rename = "full")]
    pub full: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "n_users")]
    pub n_users: f32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nonce", skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "permanent")]
    pub permanent: bool,
    #[serde(rename = "photonRegion")]
    pub photon_region: String,
    #[serde(rename = "platforms")]
    pub platforms: Box<crate::models::InstancePlatforms>,
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    pub _type: String,
    /// Always empty on non-existing instances, and non-present on existing instances.
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<serde_json::Value>>,
    /// Only present on non-existing instances, and only contains a very small subject of World object. Use World API instead.
    #[serde(rename = "world", skip_serializing_if = "Option::is_none")]
    pub world: Option<serde_json::Value>,
    #[serde(rename = "worldId")]
    pub world_id: String,
}

impl Instance {
    pub fn new(active: bool, can_request_invite: bool, capacity: f32, client_number: String, full: bool, id: String, instance_id: String, location: String, n_users: f32, name: String, owner_id: String, permanent: bool, photon_region: String, platforms: crate::models::InstancePlatforms, region: String, short_name: String, tags: Vec<String>, _type: String, world_id: String) -> Instance {
        Instance {
            active,
            can_request_invite,
            capacity,
            client_number,
            full,
            id,
            instance_id,
            location,
            n_users,
            name,
            nonce: None,
            owner_id,
            permanent,
            photon_region,
            platforms: Box::new(platforms),
            region,
            short_name,
            tags,
            _type,
            users: None,
            world: None,
            world_id,
        }
    }
}

