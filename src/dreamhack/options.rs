use super::ToRequestString;
use ratatui::prelude::*;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    All,
    Unranked,
    Beginner,
    LEVEL1,
    LEVEL2,
    LEVEL3,
    LEVEL4,
    LEVEL5,
    LEVEL6,
    LEVEL7,
    LEVEL8,
    LEVEL9,
    LEVEL10,
}

impl From<u64> for Difficulty {
    fn from(value: u64) -> Self {
        match value {
            0 => Difficulty::Unranked,
            1 => Difficulty::LEVEL1,
            2 => Difficulty::LEVEL2,
            3 => Difficulty::LEVEL3,
            4 => Difficulty::LEVEL4,
            5 => Difficulty::LEVEL5,
            6 => Difficulty::LEVEL6,
            7 => Difficulty::LEVEL7,
            8 => Difficulty::LEVEL8,
            9 => Difficulty::LEVEL9,
            10 => Difficulty::LEVEL10,
            _ => Difficulty::All,
        }
    }
}

impl From<Difficulty> for Option<u64> {
    fn from(value: Difficulty) -> Self {
        match value {
            Difficulty::All => None,
            Difficulty::LEVEL1 | Difficulty::Beginner => Some(1),
            Difficulty::LEVEL2 => Some(2),
            Difficulty::LEVEL3 => Some(3),
            Difficulty::LEVEL4 => Some(4),
            Difficulty::LEVEL5 => Some(5),
            Difficulty::LEVEL6 => Some(6),
            Difficulty::LEVEL7 => Some(7),
            Difficulty::LEVEL8 => Some(8),
            Difficulty::LEVEL9 => Some(9),
            Difficulty::LEVEL10 => Some(10),
            Difficulty::Unranked => Some(0),
        }
    }
}

impl FromIndex<Difficulty> for Difficulty {
    fn from_index(index: usize) -> Difficulty {
        match index {
            0 => Difficulty::All,
            1 => Difficulty::LEVEL1,
            2 => Difficulty::LEVEL2,
            3 => Difficulty::LEVEL3,
            4 => Difficulty::LEVEL4,
            5 => Difficulty::LEVEL5,
            6 => Difficulty::LEVEL6,
            7 => Difficulty::LEVEL7,
            8 => Difficulty::LEVEL8,
            9 => Difficulty::LEVEL9,
            10 => Difficulty::LEVEL10,
            _ => Difficulty::All,
        }
    }
}

impl Variants<Difficulty> for Difficulty {
    fn variants() -> &'static [Difficulty] {
        &[
            Difficulty::All,
            Difficulty::LEVEL1,
            Difficulty::LEVEL2,
            Difficulty::LEVEL3,
            Difficulty::LEVEL4,
            Difficulty::LEVEL5,
            Difficulty::LEVEL6,
            Difficulty::LEVEL7,
            Difficulty::LEVEL8,
            Difficulty::LEVEL9,
            Difficulty::LEVEL10,
        ]
    }
}

impl ToColorString for Difficulty {
    fn to_color_string(&self) -> Span {
        match self {
            Difficulty::LEVEL1 => Span::styled(
                "LEVEL 1",
                Style::default().fg(Color::from_str("#58d68d").unwrap()),
            ),
            Difficulty::LEVEL2 => Span::styled(
                "LEVEL 2",
                Style::default().fg(Color::from_str("#2ecc71").unwrap()),
            ),
            Difficulty::LEVEL3 => Span::styled(
                "LEVEL 3",
                Style::default().fg(Color::from_str("#28b463").unwrap()),
            ),
            Difficulty::LEVEL4 => Span::styled(
                "LEVEL 4",
                Style::default().fg(Color::from_str("#5dade2").unwrap()),
            ),
            Difficulty::LEVEL5 => Span::styled(
                "LEVEL 5",
                Style::default().fg(Color::from_str("#3498db").unwrap()),
            ),
            Difficulty::LEVEL6 => Span::styled(
                "LEVEL 6",
                Style::default().fg(Color::from_str("#2e86c1").unwrap()),
            ),
            Difficulty::LEVEL7 => Span::styled(
                "LEVEL 7",
                Style::default().fg(Color::from_str("#0000CD").unwrap()),
            ),
            Difficulty::LEVEL8 => Span::styled(
                "LEVEL 8",
                Style::default().fg(Color::from_str("#00008B").unwrap()),
            ),
            Difficulty::LEVEL9 => Span::styled(
                "LEVEL 9",
                Style::default().fg(Color::from_str("#FF0000").unwrap()),
            ),
            Difficulty::LEVEL10 => Span::styled(
                "LEVEL 10",
                Style::default().fg(Color::from_str("#DC143C").unwrap()),
            ),
            Difficulty::Unranked => Span::styled(
                "Unranked",
                Style::default().fg(Color::from_str("#696969").unwrap()),
            ),
            Difficulty::Beginner => Span::styled(
                "Beginner",
                Style::default().fg(Color::from_str("#7FFF00").unwrap()),
            ),
            Difficulty::All => Span::styled(
                "All",
                Style::default().fg(Color::from_str("#696969").unwrap()),
            ),
        }
    }
}

impl ToRequestString for Difficulty {
    fn to_request_string(&self) -> String {
        match self {
            Difficulty::LEVEL1 => "1".to_string(),
            Difficulty::LEVEL2 => "2".to_string(),
            Difficulty::LEVEL3 => "3".to_string(),
            Difficulty::LEVEL4 => "4".to_string(),
            Difficulty::LEVEL5 => "5".to_string(),
            Difficulty::LEVEL6 => "6".to_string(),
            Difficulty::LEVEL7 => "7".to_string(),
            Difficulty::LEVEL8 => "8".to_string(),
            Difficulty::LEVEL9 => "9".to_string(),
            Difficulty::LEVEL10 => "10".to_string(),
            Difficulty::Unranked => "0".to_string(),
            Difficulty::Beginner => "1".to_string(),
            Difficulty::All => "".to_string(),
        }
    }
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Difficulty::LEVEL1 => write!(f, "LEVEL 1"),
            Difficulty::LEVEL2 => write!(f, "LEVEL 2"),
            Difficulty::LEVEL3 => write!(f, "LEVEL 3"),
            Difficulty::LEVEL4 => write!(f, "LEVEL 4"),
            Difficulty::LEVEL5 => write!(f, "LEVEL 5"),
            Difficulty::LEVEL6 => write!(f, "LEVEL 6"),
            Difficulty::LEVEL7 => write!(f, "LEVEL 7"),
            Difficulty::LEVEL8 => write!(f, "LEVEL 8"),
            Difficulty::LEVEL9 => write!(f, "LEVEL 9"),
            Difficulty::LEVEL10 => write!(f, "LEVEL 10"),
            Difficulty::Unranked => write!(f, "Unranked"),
            Difficulty::Beginner => write!(f, "Beginner"),
            Difficulty::All => write!(f, "All"),
        }
    }
}

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Category {
    All,
    Pwnable,
    Reversing,
    Web,
    Crypto,
}

impl FromIndex<Category> for Category {
    fn from_index(index: usize) -> Category {
        match index {
            0 => Category::All,
            1 => Category::Pwnable,
            2 => Category::Reversing,
            3 => Category::Web,
            4 => Category::Crypto,
            _ => Category::All,
        }
    }
}

impl Variants<Category> for Category {
    fn variants() -> &'static [Category] {
        &[
            Category::All,
            Category::Pwnable,
            Category::Reversing,
            Category::Web,
            Category::Crypto,
        ]
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Category::All => write!(f, "All"),
            Category::Pwnable => write!(f, "Pwnable"),
            Category::Reversing => write!(f, "Reversing"),
            Category::Web => write!(f, "Web"),
            Category::Crypto => write!(f, "Crypto"),
        }
    }
}

impl ToRequestString for Category {
    fn to_request_string(&self) -> String {
        match self {
            Category::All => "".to_string(),
            Category::Pwnable => "pwnable".to_string(),
            Category::Reversing => "reversing".to_string(),
            Category::Web => "web".to_string(),
            Category::Crypto => "crypto".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    ToDo,
    All,
    Attempted,
    Solved,
}

impl FromIndex<Status> for Status {
    fn from_index(index: usize) -> Status {
        match index {
            0 => Status::ToDo,
            1 => Status::All,
            2 => Status::Attempted,
            3 => Status::Solved,
            _ => Status::All,
        }
    }
}

impl Variants<Status> for Status {
    fn variants() -> &'static [Status] {
        &[Status::ToDo, Status::All, Status::Attempted, Status::Solved]
    }
}

impl ToRequestString for Status {
    fn to_request_string(&self) -> String {
        match self {
            Status::ToDo => "todo".to_string(),
            Status::All => "".to_string(),
            Status::Attempted => "attempted".to_string(),
            Status::Solved => "solved".to_string(),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Status::ToDo => write!(f, "To Do"),
            Status::All => write!(f, "All"),
            Status::Attempted => write!(f, "Attempted"),
            Status::Solved => write!(f, "Solved"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orderings {
    Newist,
    MostSolved,
    LeastSolved,
}

impl FromIndex<Orderings> for Orderings {
    fn from_index(index: usize) -> Orderings {
        match index {
            0 => Orderings::Newist,
            1 => Orderings::MostSolved,
            2 => Orderings::LeastSolved,
            _ => Orderings::Newist,
        }
    }
}

impl Variants<Orderings> for Orderings {
    fn variants() -> &'static [Orderings] {
        &[
            Orderings::Newist,
            Orderings::MostSolved,
            Orderings::LeastSolved,
        ]
    }
}

impl ToRequestString for Orderings {
    fn to_request_string(&self) -> String {
        match self {
            Orderings::Newist => "".to_string(),
            Orderings::MostSolved => "-cnt_solvers".to_string(),
            Orderings::LeastSolved => "cnt_solvers".to_string(),
        }
    }
}

impl Display for Orderings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Orderings::Newist => write!(f, "Newist"),
            Orderings::MostSolved => write!(f, "Most Solved"),
            Orderings::LeastSolved => write!(f, "Least Solved"),
        }
    }
}
