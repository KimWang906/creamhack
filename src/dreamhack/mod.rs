use ratatui::text::Span;

pub mod options;

#[allow(dead_code)]
const CHALLENGES_URL: &str = "https://dreamhack.io/api/v1/wargame/challenges/";

#[allow(dead_code)]
const LOGIN_URL: &str = "https://dreamhack.io/api/v1/auth/login/";

pub trait FromIndex<T> {
    fn from_index(index: usize) -> T;
}

pub trait Variants<T> {
    fn variants() -> &'static [T];
}

pub trait ToRequestString {
    fn to_request_string(&self) -> String;
}

pub trait ToColorString {
    fn to_color_string(&self) -> Span;
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
        use ratatui::text::{Line, Text};

        /* Handler for Challenge */
        use super::{ChallengeResponseData, ToColorString};
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
