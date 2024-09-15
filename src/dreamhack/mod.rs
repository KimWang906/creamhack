use ratatui::text::Span;

pub mod options;

#[allow(dead_code)]
const CHALLENGES_URL: &str = "https://dreamhack.io/api/v1/wargame/challenges/";

#[allow(dead_code)]
const LOGIN_URL: &str = "https://dreamhack.io/api/v1/auth/login/";

pub trait ToRequestString {
    fn to_request_string(&self) -> String;
}

pub trait ToColorString {
    fn to_color_string(&self) -> Span;
}

pub mod auth {
    use serde::{Deserialize, Serialize};

    use super::LOGIN_URL;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Login {
        email: String,
        password: String,
        save_login: bool,
    }

    #[derive(Debug, Clone, Default)]
    pub struct AuthKey(String);

    #[derive(Debug, Clone, Default)]
    pub struct AuthCookies {
        csrf_token: String,
        sessionid: String,
    }

    #[derive(Debug, Clone, Default)]
    pub struct Auth {
        key: AuthKey,
        cookies: AuthCookies,
    }

    impl AuthCookies {
        pub fn to_request(&self) -> String {
            format!(
                "csrf_token={}; sessionid={}",
                self.csrf_token, self.sessionid
            )
        }

        pub fn get_csrf_token(&self) -> &str {
            &self.csrf_token
        }
    }

    impl Auth {
        pub fn get_key(&self) -> &str {
            &self.key.0
        }

        pub fn get_cookies(&self) -> &AuthCookies {
            &self.cookies
        }

        pub fn send_login(email: &str, password: &str, save_login: bool) -> Option<Auth> {
            let login = Login {
                email: email.to_owned(),
                password: password.to_owned(),
                save_login,
            };

            match reqwest::blocking::Client::new()
                .post(LOGIN_URL)
                .json(&login)
                .send()
            {
                Ok(response) => {
                    let mut cookies = AuthCookies::default();
                    for cookie in response.cookies() {
                        match cookie.name() {
                            "csrf_token" => {
                                cookies.csrf_token = cookie.value().to_owned();
                            }
                            "sessionid" => {
                                cookies.sessionid = cookie.value().to_owned();
                            }
                            _ => {}
                        }
                    }

                    let key = AuthKey(
                        response
                            .json::<serde_json::Value>()
                            .unwrap()
                            .get("key")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_owned(),
                    );

                    Some(Auth { key, cookies })
                }
                Err(e) => {
                    dbg!(e);
                    None
                } // TODO: Error Handling
            }
        }
    }
}

pub mod challenge {
    #![allow(dead_code)]

    use handle::{Challenge, PageInfo};
    use options::{Category, Difficulty, Orderings, Status};
    use reqwest::Url;
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ChallengeResponseData {
        id: u64,
        hit_count: HitCount,
        exported_from: Option<serde_json::Value>,
        cnt_solvers: u64,
        cnt_writeups: u64,
        cnt_vote: u64,
        cnt_questions: u64,
        cnt_comments: u64,
        is_completed: bool,
        is_attempted: bool,
        is_difficulty_voted: bool,
        difficulty_display: String,
        partners: Vec<Option<serde_json::Value>>,
        has_author_writeup: bool,
        tags: Vec<String>,
        author: Author,
        repository: String,
        title: String,
        description: String,
        public: String,
        public_expires: String,
        needs_vm: bool,
        deployed: String,
        difficulty: u64,
        official: bool,
        is_beginner: bool,
        is_public: bool,
        is_featured: bool,
        created_at: String,
        public_at: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Author {
        id: u64,
        nickname: String,
        profile_image: Option<String>,
        is_staff: bool,
        representative: Option<serde_json::Value>,
        introduction: Option<String>,
        country: Option<String>,
        ctf: Option<serde_json::Value>,
        wargame: Wargame,
        contributions: Contributions,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Contributions {
        level: u64,
        exp: u64,
        total_exp: u64,
        exp_needed: u64,
        updated_at: String,
        rank: u64,
        totals: u64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Wargame {
        rank: u64,
        score: u64,
        category: CategoryInfo,
        last_solved_at: Option<serde_json::Value>,
    }

    // 차후 업데이트에 따라 필드가 추가될 수 있습니다.
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct CategoryInfo {
        pwnable: AuthorWargameCategory,
        reversing: AuthorWargameCategory,
        web: AuthorWargameCategory,
        crypto: AuthorWargameCategory,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct AuthorWargameCategory {
        score: u64,
        rank: u64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct HitCount {
        hits: u64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ChallengeListResponse {
        count: u32,
        page_size: u32,
        next: Option<String>,
        previous: Option<String>,
        pub results: Vec<ChallengeResponseData>,
    }

    pub struct RequestChallengeList {
        options: ChallengeOptions,
    }

    pub struct ChallengeOptions {
        page: u64,
        search: Option<String>,
        ordering: Option<Orderings>,
        scope: Option<String>,
        category: Option<Category>,
        difficulty: Option<Difficulty>,
        type_value: Option<String>,
        status: Option<Status>,
        page_size: Option<u64>,
    }

    impl RequestChallengeList {
        pub fn new() -> Self {
            RequestChallengeList {
                options: ChallengeOptions {
                    page: 1,
                    search: None,
                    ordering: None,
                    scope: None,
                    category: None,
                    difficulty: None,
                    type_value: None,
                    status: None,
                    page_size: None,
                },
            }
        }

        pub fn set_category(&mut self, category: Category) {
            self.options.category = Some(category);
        }

        pub fn set_difficulty(&mut self, difficulty: Difficulty) {
            self.options.difficulty = Some(difficulty);
        }

        pub fn set_status(&mut self, status: Status) {
            self.options.status = Some(status);
        }

        pub fn set_ordering(&mut self, ordering: Orderings) {
            self.options.ordering = Some(ordering);
        }

        pub fn set_page(&mut self, page: u64) {
            self.options.page = page;
        }

        pub fn set_search(&mut self, search: String) {
            self.options.search = Some(search);
        }

        pub fn set_page_size(&mut self, page_size: u64) {
            self.options.page_size = Some(page_size);
        }

        pub fn send_request(&mut self) -> Option<(Vec<Challenge>, PageInfo)> {
            let url = Url::parse_with_params(
                CHALLENGES_URL,
                [
                    (
                        "ordering",
                        self.options
                            .ordering
                            .as_ref()
                            .unwrap_or(&Orderings::Newist)
                            .to_request_string(),
                    ),
                    (
                        "category",
                        self.options
                            .category
                            .as_ref()
                            .unwrap_or(&Category::All)
                            .to_request_string(),
                    ),
                    (
                        "status",
                        self.options
                            .status
                            .as_ref()
                            .unwrap_or(&Status::All)
                            .to_request_string(),
                    ),
                    (
                        "difficulty",
                        self.options
                            .difficulty
                            .as_ref()
                            .unwrap_or(&Difficulty::All)
                            .to_request_string(),
                    ),
                    ("page", self.options.page.to_string()),
                    (
                        "search",
                        self.options
                            .search
                            .as_ref()
                            .unwrap_or(&"".to_owned())
                            .to_owned(),
                    ), //
                    ("type", "".to_string()),  // type 기능은 미구현
                    ("scope", "".to_string()), // scope 기능은 미구현
                    (
                        "page_size",
                        self.options.page_size.unwrap_or(20).to_string(),
                    ),
                ],
            )
            .expect("Failed to parse URL");

            match serde_json::from_str::<ChallengeListResponse>(
                reqwest::blocking::get(url)
                    .unwrap()
                    .text()
                    .unwrap_or_default()
                    .as_str(),
            ) {
                Ok(response) => {
                    let challenges = response
                        .results
                        .iter()
                        .map(|challenge| Challenge::from(challenge.clone()))
                        .collect::<Vec<Challenge>>();

                    let page_info = PageInfo {
                        page_index: self.options.page,
                        count: response.count,
                        page_size: response.page_size,
                        next: response.next,
                        previous: response.previous,
                    };

                    Some((challenges, page_info))
                }
                Err(_e) => None, // TODO: Error Handling
            }
        }
    }

    pub mod handle {

        use core::panic;

        use anyhow::Context;
        use ratatui::text::{Line, Text};
        use reqwest::{
            blocking::Client,
            header::{HeaderMap, HeaderValue, COOKIE},
        };

        /* Handler for Challenge */
        use super::{auth::Auth, vm_info::MachineInfo, ChallengeResponseData, ToColorString};
        use crate::dreamhack::options::Difficulty;

        #[derive(Debug, Clone, Default, PartialEq, Eq)]
        pub struct PageInfo {
            pub(super) page_index: u64,
            pub(super) count: u32,
            pub(super) page_size: u32,
            pub(super) next: Option<String>,
            pub(super) previous: Option<String>,
        }

        impl PageInfo {
            pub fn get_count(&self) -> u32 {
                self.count
            }

            pub fn get_page_size(&self) -> u32 {
                self.page_size
            }

            pub fn get_next(&self) -> &Option<String> {
                &self.next
            }

            pub fn get_previous(&self) -> &Option<String> {
                &self.previous
            }

            pub fn has_next(&self) -> bool {
                self.next.is_some()
            }

            pub fn has_previous(&self) -> bool {
                self.previous.is_some()
            }

            pub fn get_page_idx(&self) -> u64 {
                self.page_index
            }

            pub fn next_page(&mut self) {
                if self.has_next() {
                    self.page_index += 1
                }
            }

            pub fn previous_page(&mut self) {
                if self.has_previous() {
                    self.page_index -= 1
                }
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct ChallengeInfo {
            title: String,
            description: String,
            difficulty: u64,
            author: String,
            tags: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct ChallengeMetadata {
            repository: String,
            public: String,
            flags: Flags,
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Flags {
            is_completed: bool,
            is_attempted: bool,
            is_difficulty_voted: bool,
            is_beginner: bool,
            is_public: bool,
            is_featured: bool,
            has_author_writeup: bool,
            needs_vm: bool,
            official: bool,
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Challenge {
            id: u64,
            info: ChallengeInfo,
            metadata: ChallengeMetadata,
        }

        impl Challenge {
            pub fn get_id(&self) -> u64 {
                self.id
            }

            pub fn get_info(&self) -> &ChallengeInfo {
                &self.info
            }

            pub fn get_metadata(&self) -> &ChallengeMetadata {
                &self.metadata
            }

            pub fn download_challenge(&self) -> Vec<u8> {
                let response = reqwest::blocking::get(self.metadata.get_public())
                    .expect("Failed to download challenge file")
                    .bytes()
                    .expect("Failed to get bytes from response");

                response.to_vec()
            }

            pub fn create_vm(&self, auth: &Auth) -> bool {
                if auth.get_key().is_empty() {
                    #[cfg(debug_assertions)]
                    log::error!("AuthKey is empty");
                    return false;
                }

                let mut headers = HeaderMap::new();
                headers.insert(
                    COOKIE,
                    HeaderValue::from_str(&format!(
                        "i18n_redirected=ko; {}",
                        auth.get_cookies().to_request()
                    ))
                    .unwrap(),
                );

                headers.insert(
                    "X-Csrftoken",
                    HeaderValue::from_str(auth.get_cookies().get_csrf_token()).unwrap(),
                );

                let request = Client::new()
                    .post(format!(
                        "https://dreamhack.io/api/v1/wargame/challenges/{}/live/",
                        self.id
                    ))
                    .bearer_auth(auth.get_key())
                    .headers(headers);

                #[cfg(debug_assertions)]
                log::info!("Request: {:?}", request);

                let response = request.send().context("Failed to create VM").unwrap();

                let status = response.status();
                #[cfg(debug_assertions)]
                let response_text = response
                    .text()
                    .unwrap_or_else(|_| "Failed to read response body".to_string());

                #[cfg(debug_assertions)]
                if !status.is_success() {
                    log::error!("Failed to create VM, status: {}", status);
                    log::error!("Response: {:?}", response_text);
                } else {
                    log::info!("VM created successfully");
                    log::info!("Response: {:?}", response_text);
                }

                status.is_success()
            }

            pub fn get_vm_info(&self, auth: &Auth) -> MachineInfo {
                if auth.get_key().is_empty() {
                    #[cfg(debug_assertions)]
                    log::error!("AuthKey is empty");
                    panic!("AuthKey is empty");
                }

                let mut headers = HeaderMap::new();
                headers.insert(
                    COOKIE,
                    HeaderValue::from_str(&format!(
                        "i18n_redirected=ko; {}",
                        auth.get_cookies().to_request()
                    ))
                    .unwrap(),
                );

                let request = Client::new()
                    .get(format!(
                        "https://dreamhack.io/api/v1/wargame/challenges/{}/live/",
                        self.id
                    ))
                    .bearer_auth(auth.get_key())
                    .headers(headers);

                #[cfg(debug_assertions)]
                log::info!("Request: {:?}", request);

                let response = request.send().context("Failed to get VM info").unwrap();

                let text = response.text();

                #[cfg(debug_assertions)]
                log::info!("Response: {:?}", text);

                serde_json::from_str::<MachineInfo>(
                    text.context("Failed to get text from response")
                        .unwrap()
                        .as_str(),
                )
                .context("Failed to parse JSON")
                .unwrap()
            }

            pub fn submit_flag(&self, auth: Auth, flag: &str) -> bool {
                if auth.get_key().is_empty() {
                    #[cfg(debug_assertions)]
                    log::error!("AuthKey is empty");
                    panic!("AuthKey is empty");
                }

                let mut headers = HeaderMap::new();
                headers.insert(
                    COOKIE,
                    HeaderValue::from_str(&format!(
                        "i18n_redirected=ko; {}",
                        auth.get_cookies().to_request()
                    ))
                    .unwrap(),
                );

                let request = reqwest::blocking::Client::new()
                    .post(format!(
                        "https://dreamhack.io/api/v1/wargame/challenges/{}/auth",
                        self.id
                    ))
                    .bearer_auth(auth.get_key())
                    .json(&serde_json::json!({
                        "flag": flag
                    }));

                let response = request.send().context("Failed to send flag").unwrap();

                let status = response.status();
                #[cfg(debug_assertions)]
                let response_text = response
                    .text()
                    .unwrap_or_else(|_| "Failed to read response body".to_string());

                #[cfg(debug_assertions)]
                if !status.is_success() {
                    log::error!("Failed to create VM, status: {}", status);
                    log::error!("Response: {:?}", response_text);
                } else {
                    log::info!("VM created successfully");
                }

                status.is_success()
            }
        }

        impl ChallengeInfo {
            /// title of challenge
            pub fn get_title(&self) -> &str {
                &self.title
            }

            /// description of challenge
            pub fn get_description(&self) -> &str {
                &self.description
            }

            /// difficulty is u64, so convert to Difficulty enum
            pub fn get_difficulty(&self) -> Difficulty {
                Difficulty::from(self.difficulty)
            }

            /// author info
            pub fn get_author(&self) -> &str {
                &self.author
            }

            /// category of challenge
            pub fn get_tags(&self) -> &Vec<String> {
                &self.tags
            }
        }
        pub trait ToSimpleInfo {
            fn to_simple_info(&self) -> Text;
        }

        impl ToSimpleInfo for Challenge {
            fn to_simple_info(&self) -> Text {
                let info = self.get_info();
                let content = vec![
                    Line::raw(format!("Title: {}\n", info.get_title())),
                    Line::raw(format!(
                        "Level: {}\n",
                        info.get_difficulty().to_color_string()
                    )),
                    Line::raw(format!("Author: {}\n", info.get_author())),
                    Line::raw("\n"),
                ];

                Text::from(content)
            }
        }

        pub trait ToDetailedInfo {
            fn to_detailed_info(&self) -> Text;
        }

        impl ToDetailedInfo for Challenge {
            fn to_detailed_info(&self) -> Text {
                let info = self.get_info();

                let content = vec![
                    Line::raw(format!("Title: {}\n", info.get_title())),
                    Line::raw(format!(
                        "Level: {}\n",
                        info.get_difficulty().to_color_string()
                    )),
                    Line::raw(format!("Author: {}\n", info.get_author())),
                    Line::raw(format!("Description: {}\n", info.get_description())),
                    Line::raw(format!("Tags: {:?}\n", info.get_tags())),
                    Line::raw("\n"),
                ];

                Text::from(content)
            }
        }

        impl ChallengeMetadata {
            /// repository name is title of challenge(whitespace replaced with '_')
            pub fn get_repository(&self) -> &str {
                &self.repository
            }

            /// public url of challenge(file link)
            pub fn get_public(&self) -> &str {
                &self.public
            }
        }

        impl From<ChallengeResponseData> for Challenge {
            fn from(challenge: ChallengeResponseData) -> Self {
                Challenge {
                    id: challenge.id,
                    info: ChallengeInfo {
                        title: challenge.title,
                        description: challenge.description,
                        difficulty: challenge.difficulty,
                        tags: challenge.tags,
                        author: challenge.author.nickname,
                    },
                    metadata: ChallengeMetadata {
                        repository: challenge.repository,
                        public: challenge.public,
                        flags: Flags {
                            is_completed: challenge.is_completed,
                            is_attempted: challenge.is_attempted,
                            is_difficulty_voted: challenge.is_difficulty_voted,
                            is_beginner: challenge.is_beginner,
                            is_public: challenge.is_public,
                            is_featured: challenge.is_featured,
                            has_author_writeup: challenge.has_author_writeup,
                            needs_vm: challenge.needs_vm,
                            official: challenge.official,
                        },
                    },
                }
            }
        }
    }
}

pub mod vm_info {
    use anyhow::Context;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct MachineInfo {
        id: String,
        state: String,
        memory: i64,
        swap: i64,
        starttime: String,
        endtime: String,
        cputime: f64,
        host: String,
        port_mappings: Vec<Vec<PortMapping>>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum PortMapping {
        Integer(i64),
        String(String),
    }

    impl PortMapping {
        pub fn get_port(&self) -> u16 {
            match self {
                PortMapping::Integer(port) => *port as u16,
                _ => unreachable!(),
            }
        }

        pub fn get_protocol(&self) -> String {
            match self {
                PortMapping::String(protocol) => protocol.clone(),
                _ => unreachable!(),
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub enum Protocol {
        #[default]
        None,
        Tcp,
        Udp,
    }

    impl Protocol {
        fn from_str(protocol: &str) -> Self {
            match protocol {
                "tcp" => Protocol::Tcp,
                "udp" => Protocol::Udp,
                _ => unreachable!(),
            }
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct NetworkInfo {
        pub protocol: Protocol,
        pub host: String,
        pub external: u16,
        pub internal: u16,
    }

    impl MachineInfo {
        /// port mappings of machine.
        ///
        /// example: "port_mappings":[["tcp",10332,8080]]
        pub fn get_network_info(&self) -> Option<NetworkInfo> {
            let info = self
                .port_mappings
                .first()
                .context("Failed to get port mappings")
                .ok();
            info.map(|info| NetworkInfo {
                protocol: Protocol::from_str(
                    &info
                        .first()
                        .context("Failed to get protocol")
                        .unwrap()
                        .get_protocol(),
                ),
                host: self.host.clone(),
                external: info
                    .get(1)
                    .context("Failed to get external port")
                    .unwrap()
                    .get_port(),
                internal: info
                    .get(2)
                    .context("Failed to get internal port")
                    .unwrap()
                    .get_port(),
            })
        }
    }

    impl NetworkInfo {
        pub fn get_uri_pwn(&self) -> String {
            format!("{}:{}", self.host, self.external)
        }

        pub fn get_uri_web(&self) -> String {
            format!("http://{}:{}/", self.host, self.external)
        }
    }
}
