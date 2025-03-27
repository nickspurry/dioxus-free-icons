use super::super::IconShape;
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsAccount;
impl IconShape for VsAccount {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 7.992C16 3.58 12.416 0 8 0S0 3.58 0 7.992c0 2.43 1.104 4.62 2.832 6.09.016.016.032.016.032.032.144.112.288.224.448.336.08.048.144.111.224.175A7.98 7.98 0 0 0 8.016 16a7.98 7.98 0 0 0 4.48-1.375c.08-.048.144-.111.224-.16.144-.111.304-.223.448-.335.016-.016.032-.016.032-.032 1.696-1.487 2.8-3.676 2.8-6.106zm-8 7.001c-1.504 0-2.88-.48-4.016-1.279.016-.128.048-.255.08-.383a4.17 4.17 0 0 1 .416-.991c.176-.304.384-.576.64-.816.24-.24.528-.463.816-.639.304-.176.624-.304.976-.4A4.15 4.15 0 0 1 8 10.342a4.185 4.185 0 0 1 2.928 1.166c.368.368.656.8.864 1.295.112.288.192.592.24.911A7.03 7.03 0 0 1 8 14.993zm-2.448-7.4a2.49 2.49 0 0 1-.208-1.024c0-.351.064-.703.208-1.023.144-.32.336-.607.576-.847.24-.24.528-.431.848-.575.32-.144.672-.208 1.024-.208.368 0 .704.064 1.024.208.32.144.608.336.848.575.24.24.432.528.576.847.144.32.208.672.208 1.023 0 .368-.064.704-.208 1.023a2.84 2.84 0 0 1-.576.848 2.84 2.84 0 0 1-.848.575 2.715 2.715 0 0 1-2.064 0 2.84 2.84 0 0 1-.848-.575 2.526 2.526 0 0 1-.56-.848zm7.424 5.306c0-.032-.016-.048-.016-.08a5.22 5.22 0 0 0-.688-1.406 4.883 4.883 0 0 0-1.088-1.135 5.207 5.207 0 0 0-1.04-.608 2.82 2.82 0 0 0 .464-.383 4.2 4.2 0 0 0 .624-.784 3.624 3.624 0 0 0 .528-1.934 3.71 3.71 0 0 0-.288-1.47 3.799 3.799 0 0 0-.816-1.199 3.845 3.845 0 0 0-1.2-.8 3.72 3.72 0 0 0-1.472-.287 3.72 3.72 0 0 0-1.472.288 3.631 3.631 0 0 0-1.2.815 3.84 3.84 0 0 0-.8 1.199 3.71 3.71 0 0 0-.288 1.47c0 .352.048.688.144 1.007.096.336.224.64.4.927.16.288.384.544.624.784.144.144.304.271.48.383a5.12 5.12 0 0 0-1.04.624c-.416.32-.784.703-1.088 1.119a4.999 4.999 0 0 0-.688 1.406c-.016.032-.016.064-.016.08C1.776 11.636.992 9.91.992 7.992.992 4.14 4.144.991 8 .991s7.008 3.149 7.008 7.001a6.96 6.96 0 0 1-2.032 4.907z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsActivateBreakpoints;
impl IconShape for VsActivateBreakpoints {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 5.5a4.394 4.394 0 0 1-4 4.5 2.955 2.955 0 0 0-.2-1A3.565 3.565 0 0 0 14 5.5a3.507 3.507 0 0 0-7-.3A3.552 3.552 0 0 0 6 5a4.622 4.622 0 0 1 4.5-4A4.481 4.481 0 0 1 15 5.5zM5.5 6a4.5 4.5 0 1 0 0 9.001 4.5 4.5 0 0 0 0-9z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsAdd;
impl IconShape for VsAdd {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 7v1H8v6H7V8H1V7h6V1h1v6h6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsArchive;
impl IconShape for VsArchive {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.5 1h-13l-.5.5v3l.5.5H2v8.5l.5.5h11l.5-.5V5h.5l.5-.5v-3l-.5-.5zm-1 3H2V2h12v2h-.5zM3 13V5h10v8H3zm8-6H5v1h6V7z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsArrowBoth;
impl IconShape for VsArrowBoth {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 9l2.146 2.146-.707.708-3-3v-.708l3-3 .707.708L3 8h10l-2.146-2.146.707-.708 3 3v.708l-3 3-.707-.707L13 9H3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsArrowCircleDown;
impl IconShape for VsArrowCircleDown {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.36891 8.08074L7.50833 10.2202V4.46802H8.50833V10.1473L10.5749 8.08074L11.282 8.78784L8.32545 11.7444H7.61835L4.6618 8.78784L5.36891 8.08074Z",
            }
            path {
                d: "M14 8C14 4.68629 11.3137 2 8 2C4.68629 2 2 4.68629 2 8C2 11.3137 4.68629 14 8 14C11.3137 14 14 11.3137 14 8ZM13 8C13 10.7614 10.7614 13 8 13C5.23858 13 3 10.7614 3 8C3 5.23858 5.23858 3 8 3C10.7614 3 13 5.23858 13 8Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsArrowCircleLeft;
impl IconShape for VsArrowCircleLeft {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.91926 10.6311L5.77984 8.49167L11.532 8.49167L11.532 7.49167L5.85271 7.49167L7.91926 5.42511L7.21216 4.718L4.25561 7.67455L4.25561 8.38165L7.21216 11.3382L7.91926 10.6311Z",
            }
            path {
                d: "M8 2C11.3137 2 14 4.68629 14 8C14 11.3137 11.3137 14 8 14C4.68629 14 2 11.3137 2 8C2 4.68629 4.68629 2 8 2ZM8 3C5.23858 3 3 5.23858 3 8C3 10.7614 5.23858 13 8 13C10.7614 13 13 10.7614 13 8C13 5.23858 10.7614 3 8 3Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsArrowCircleRight;
impl IconShape for VsArrowCircleRight {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.08074 5.36891L10.2202 7.50833L4.46802 7.50833L4.46802 8.50833L10.1473 8.50833L8.08073 10.5749L8.78784 11.282L11.7444 8.32545L11.7444 7.61835L8.78784 4.6618L8.08074 5.36891Z",
            }
            path {
                d: "M8 14C4.68629 14 2 11.3137 2 8C2 4.68629 4.68629 2 8 2C11.3137 2 14 4.68629 14 8C14 11.3137 11.3137 14 8 14ZM8 13C10.7614 13 13 10.7614 13 8C13 5.23858 10.7614 3 8 3C5.23858 3 3 5.23858 3 8C3 10.7614 5.23858 13 8 13Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsArrowCircleUp;
impl IconShape for VsArrowCircleUp {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.36891 7.91926L7.50833 5.77984V11.532H8.50833V5.85271L10.5749 7.91926L11.282 7.21216L8.32545 4.25562H7.61835L4.6618 7.21216L5.36891 7.91926Z",
            }
            path {
                d: "M14 8C14 11.3137 11.3137 14 8 14C4.68629 14 2 11.3137 2 8C2 4.68629 4.68629 2 8 2C11.3137 2 14 4.68629 14 8ZM13 8C13 5.23858 10.7614 3 8 3C5.23858 3 3 5.23858 3 8C3 10.7614 5.23858 13 8 13C10.7614 13 13 10.7614 13 8Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsArrowDown;
impl IconShape for VsArrowDown {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3.147 9l5 5h.707l5-5-.707-.707L9 12.439V2H8v10.44L3.854 8.292 3.147 9z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsArrowLeft;
impl IconShape for VsArrowLeft {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7 3.093l-5 5V8.8l5 5 .707-.707-4.146-4.147H14v-1H3.56L7.708 3.8 7 3.093z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsArrowRight;
impl IconShape for VsArrowRight {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9 13.887l5-5V8.18l-5-5-.707.707 4.146 4.147H2v1h10.44L8.292 13.18l.707.707z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsArrowSmallDown;
impl IconShape for VsArrowSmallDown {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.7 8.64l-2.5 2.5h-.7L5 8.64l.7-.71 1.65 1.64V4h1v5.57L10 7.92l.7.72z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsArrowSmallLeft;
impl IconShape for VsArrowSmallLeft {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.5 10.7L4 8.2v-.7L6.5 5l.71.7-1.64 1.65h5.57v1H5.57L7.22 10l-.72.7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsArrowSmallRight;
impl IconShape for VsArrowSmallRight {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.64 5l2.5 2.5v.7l-2.5 2.5-.71-.7 1.64-1.65H4v-1h5.57L7.92 5.7l.72-.7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsArrowSmallUp;
impl IconShape for VsArrowSmallUp {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 6.5L7.5 4h.7l2.5 2.5-.7.71-1.65-1.64v5.57h-1V5.57L5.7 7.22 5 6.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsArrowSwap;
impl IconShape for VsArrowSwap {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4.207 15.061L1 11.854v-.707L4.207 7.94l.707.707-2.353 2.354H15v1H2.56l2.354 2.353-.707.707zm7.586-7L15 4.854v-.707L11.793.94l-.707.707L13.439 4H1v1h12.44l-2.354 2.354.707.707z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsArrowUp;
impl IconShape for VsArrowUp {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13.854 7l-5-5h-.707l-5 5 .707.707L8 3.561V14h1V3.56l4.146 4.147.708-.707z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsAttach;
impl IconShape for VsAttach {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.25201 15H7.75201C6.8577 14.9646 6.01378 14.5765 5.40476 13.9207C4.79575 13.2648 4.47118 12.3945 4.50201 11.5V3.682C4.47985 2.99536 4.73035 2.32785 5.19878 1.82531C5.66722 1.32278 6.31551 1.02607 7.00201 1C7.68852 1.02607 8.33681 1.32278 8.80524 1.82531C9.27367 2.32785 9.52417 2.99536 9.50201 3.682V10.849C9.51306 11.2586 9.36146 11.6559 9.08037 11.9541C8.79928 12.2522 8.41156 12.4269 8.00201 12.44C7.59635 12.4275 7.21184 12.2561 6.93129 11.9628C6.65073 11.6695 6.49657 11.2778 6.50201 10.872V7H7.50201V10.849C7.49117 10.9934 7.53752 11.1363 7.63105 11.2468C7.72458 11.3574 7.85781 11.4268 8.00201 11.44C8.14622 11.4268 8.27945 11.3574 8.37298 11.2468C8.46651 11.1363 8.51285 10.9934 8.50201 10.849V3.682C8.52443 3.26046 8.37936 2.84714 8.0984 2.53209C7.81744 2.21704 7.42336 2.02579 7.00201 2C6.58067 2.02579 6.18658 2.21704 5.90562 2.53209C5.62466 2.84714 5.47959 3.26046 5.50201 3.682V11.5C5.47146 12.1292 5.69078 12.7451 6.11222 13.2133C6.53365 13.6816 7.12304 13.9643 7.75201 14H8.25201C8.88098 13.9643 9.47038 13.6816 9.89181 13.2133C10.3132 12.7451 10.5326 12.1292 10.502 11.5V5H11.502V11.5C11.5328 12.3945 11.2083 13.2648 10.5993 13.9207C9.99025 14.5765 9.14633 14.9646 8.25201 15V15Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsAzureDevops;
impl IconShape for VsAzureDevops {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 3.62172V12.1336L11.5 15L6.075 13.025V14.9825L3.00375 10.9713L11.955 11.6704V4.00624L15 3.62172ZM12.0163 4.04994L6.99375 1V3.00125L2.3825 4.35581L1 6.12984V10.1586L2.9775 11.0325V5.86767L12.0163 4.04994Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsAzure;
impl IconShape for VsAzure {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M15.3702 13.6799L11.3702 1.67989C11.3006 1.47291 11.1652 1.29438 10.9846 1.17159C10.804 1.0488 10.5882 0.988513 10.3702 0.999896H5.63017C5.42052 0.999354 5.21598 1.0647 5.04551 1.18672C4.87504 1.30875 4.74724 1.48127 4.68015 1.67989L0.630165 13.6799C0.577646 13.8346 0.56382 13.9998 0.589943 14.1611C0.616066 14.3225 0.681335 14.4749 0.780007 14.6052C0.878678 14.7354 1.00778 14.8395 1.15598 14.9083C1.30419 14.9771 1.46699 15.0086 1.63017 14.9999H4.56016C4.76809 14.9984 4.97035 14.932 5.13883 14.8101C5.30731 14.6883 5.43363 14.5169 5.50016 14.3199L6.11015 12.5399L9.11015 14.8099C9.28448 14.9362 9.49495 15.0028 9.71018 14.9999H14.3902C14.5517 15.0052 14.7121 14.9712 14.8576 14.901C15.0032 14.8307 15.1295 14.7263 15.2259 14.5965C15.3222 14.4668 15.3856 14.3156 15.4107 14.156C15.4359 13.9963 15.422 13.833 15.3702 13.6799ZM9.75016 14.3399C9.67748 14.3399 9.60693 14.3153 9.55015 14.2699L3.90018 10.0799L3.81016 10.0099H6.81016L6.89017 9.79988L7.89017 7.26988L10.1302 13.8999C10.1482 13.9555 10.1515 14.0148 10.1399 14.072C10.1283 14.1293 10.1022 14.1826 10.064 14.2269C10.0258 14.2711 9.97689 14.3047 9.92191 14.3245C9.86694 14.3443 9.80778 14.3496 9.75016 14.3399V14.3399ZM14.4201 14.3399H10.7002C10.7749 14.1262 10.7749 13.8935 10.7002 13.6799L6.65018 1.67989H10.3702C10.4408 1.68024 10.5095 1.70258 10.5669 1.74379C10.6242 1.78501 10.6673 1.84308 10.6902 1.9099L14.7402 13.9099C14.7538 13.9597 14.756 14.012 14.7464 14.0628C14.7369 14.1136 14.7159 14.1615 14.6851 14.203C14.6542 14.2444 14.6144 14.2783 14.5685 14.302C14.5226 14.3257 14.4718 14.3387 14.4201 14.3399V14.3399Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsBeakerStop;
impl IconShape for VsBeakerStop {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.99994 15.006H8.00746C7.62983 14.7234 7.29348 14.3888 7.00908 14.0126L2.99994 14.017L4.54094 11.006H5.99997L5.99997 11C5.99997 10.6597 6.03398 10.3273 6.09878 10.006H5.04894L6.89294 6.408L6.99994 6.193V2.036L8.99994 2.012V6.007V6.249L9.07058 6.38584C9.38043 6.25613 9.7061 6.15672 10.0439 6.09131L9.99994 6.006V2.006H10.9999V1.006H9.99394V1L9.53794 1.005H4.99994V2H5.99994V5.952L2.10594 13.561C2.03023 13.7133 1.99465 13.8825 2.00254 14.0524C2.01044 14.2224 2.06156 14.3875 2.15106 14.5321C2.24057 14.6768 2.3655 14.7962 2.51404 14.8792C2.66258 14.9621 2.82982 15.0057 2.99994 15.006ZM8.77769 7.67407C9.43548 7.23455 10.2089 7 11 7C12.0608 7 13.0782 7.42149 13.8283 8.17163C14.5785 8.92178 15 9.93913 15 11C15 11.7911 14.7654 12.5645 14.3259 13.2223C13.8864 13.8801 13.2616 14.3928 12.5307 14.6956C11.7998 14.9983 10.9955 15.0774 10.2196 14.9231C9.44366 14.7688 8.73102 14.3878 8.17161 13.8284C7.6122 13.269 7.23122 12.5563 7.07688 11.7804C6.92254 11.0045 7.00167 10.2001 7.30442 9.46924C7.60717 8.73833 8.11989 8.1136 8.77769 7.67407ZM8.87864 13.1213C9.44125 13.6839 10.2043 14 11 14C11.623 14.0018 12.2312 13.8095 12.74 13.45L8.55003 9.26001C8.19046 9.76883 7.99818 10.377 7.99998 11C7.99998 11.7956 8.31603 12.5587 8.87864 13.1213ZM9.25999 8.55005L13.4499 12.74C13.8095 12.2312 14.0018 11.623 14 11C14 10.2044 13.6839 9.44127 13.1213 8.87866C12.5587 8.31605 11.7956 8 11 8C10.3769 7.9982 9.7688 8.19048 9.25999 8.55005Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsBeaker;
impl IconShape for VsBeaker {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.893 13.558L10 6.006v-4h1v-1H9.994V1l-.456.005H5V2h1v3.952l-3.894 7.609A1 1 0 0 0 3 15.006h10a1 1 0 0 0 .893-1.448zm-7-7.15L7 6.193V2.036l2-.024v4.237l.11.215 1.827 3.542H5.049l1.844-3.598zM3 14.017l1.54-3.011h6.916l1.547 3L3 14.017z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsBellDot;
impl IconShape for VsBellDot {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.9944 7.87543C12.6765 7.95676 12.3433 8 12 8C11.9965 8 11.993 8 11.9894 7.99999V8.21674C11.9894 9.12596 12.133 10.0352 12.4321 10.9085L12.803 12.0211H3.17241V12.0091L3.54328 10.8965C3.8304 10.0232 3.98593 9.114 3.98593 8.20478V6.00351C3.98593 5.44123 4.10556 4.89092 4.33287 4.38845C4.56017 3.87403 4.88318 3.41942 5.3019 3.04855C5.72062 2.66572 6.21112 2.3786 6.73751 2.21111C7.27587 2.03166 7.83815 1.97184 8.38846 2.03166C8.42817 2.03686 8.46777 2.04264 8.50725 2.04899C8.69007 1.7224 8.91737 1.42408 9.18095 1.16222C8.95341 1.10142 8.7207 1.05602 8.48417 1.02673C7.79029 0.954953 7.08445 1.02673 6.4145 1.25404C5.74455 1.46938 5.13441 1.82828 4.61999 2.30682C4.10556 2.77339 3.68684 3.34764 3.41168 3.9817C3.13652 4.61576 2.981 5.29767 2.981 6.00351V8.20478C2.981 9.00633 2.8494 9.80788 2.59817 10.5735L2 12.3441L2.47854 13.0021H5.98382C5.98382 13.5285 6.19916 14.0429 6.57002 14.4138C6.94089 14.7847 7.45532 15 7.98171 15C8.5081 15 9.02252 14.7847 9.39339 14.4138C9.76426 14.0429 9.9796 13.5285 9.9796 13.0021H13.4849L13.9634 12.3441L13.3772 10.5735C13.126 9.80788 12.9944 9.00633 12.9944 8.19282V7.87543ZM7.98171 14.019C8.2449 14.019 8.49613 13.9113 8.68755 13.7199C8.87896 13.5285 8.98663 13.2773 8.97467 13.0141H6.97678C6.97678 13.2773 7.08445 13.5285 7.27587 13.7199C7.46728 13.9113 7.71851 14.019 7.98171 14.019Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M12 7C13.6569 7 15 5.65685 15 4C15 2.34315 13.6569 1 12 1C10.3431 1 9 2.34315 9 4C9 5.65685 10.3431 7 12 7Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsBellSlashDot;
impl IconShape for VsBellSlashDot {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.61674 1.04489C8.2822 1.32437 7.98886 1.65152 7.74718 2.01591C7.40714 2.03594 7.06782 2.10101 6.73751 2.21111C6.21112 2.3786 5.72062 2.66572 5.3019 3.04855C4.88318 3.41942 4.56017 3.87403 4.33287 4.38845C4.10556 4.89092 3.98593 5.44123 3.98593 6.00351V8.20478C3.98593 8.81957 3.91482 9.43435 3.78 10.038L2.26841 11.5496L2.59817 10.5735C2.8494 9.80788 2.981 9.00633 2.981 8.20478V6.00351C2.981 5.29767 3.13653 4.61576 3.41168 3.9817C3.68684 3.34764 4.10556 2.77339 4.61999 2.30682C5.13442 1.82828 5.74455 1.46938 6.4145 1.25404C7.08445 1.02673 7.79029 0.954953 8.48417 1.02673C8.5285 1.03222 8.57269 1.03828 8.61674 1.04489ZM13.0142 8.73891C12.6984 8.8517 12.3664 8.93021 12.0226 8.96998C12.0807 9.62589 12.2157 10.2766 12.4321 10.9085L12.803 12.0211H6.31835L5.33735 13.0021H5.98382C5.98382 13.5285 6.19916 14.0429 6.57002 14.4138C6.94089 14.7847 7.45532 15 7.98171 15C8.5081 15 9.02252 14.7847 9.39339 14.4138C9.76426 14.0429 9.9796 13.5285 9.9796 13.0021H13.4849L13.9634 12.3441L13.3772 10.5735C13.1824 9.97985 13.0595 9.3646 13.0142 8.73891ZM8.68755 13.7199C8.49613 13.9113 8.2449 14.019 7.98171 14.019C7.71851 14.019 7.46728 13.9113 7.27587 13.7199C7.08445 13.5285 6.97678 13.2773 6.97678 13.0141H8.97467C8.98663 13.2773 8.87896 13.5285 8.68755 13.7199Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M15.2486 1.66567C15.0528 1.39375 14.8238 1.1474 14.5674 0.932596L15.1421 0.35791L15.8492 1.06502L15.2486 1.66567ZM8.9326 6.56745L1 14.5L1.70711 15.2072L9.66567 7.24859C9.39375 7.05285 9.1474 6.82381 8.9326 6.56745Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M12 7C13.6569 7 15 5.65685 15 4C15 2.34315 13.6569 1 12 1C10.3431 1 9 2.34315 9 4C9 5.65685 10.3431 7 12 7Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsBellSlash;
impl IconShape for VsBellSlash {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.0268 2.08559C10.2949 1.51028 9.41936 1.14252 8.48416 1.02673C7.79028 0.954953 7.08444 1.02673 6.41449 1.25404C5.74454 1.46938 5.13441 1.82828 4.61998 2.30682C4.10555 2.77339 3.68683 3.34764 3.41168 3.9817C3.13652 4.61576 2.98099 5.29767 2.98099 6.00351V8.20478C2.98099 8.97587 2.85921 9.74697 2.62628 10.4861L3.92646 9.18593C3.96573 8.86 3.98592 8.53239 3.98592 8.20478V6.00351C3.98592 5.44123 4.10555 4.89092 4.33286 4.38845C4.56016 3.87403 4.88318 3.41942 5.30189 3.04855C5.72061 2.66572 6.21111 2.3786 6.7375 2.21111C7.27586 2.03166 7.83814 1.97184 8.38846 2.03166C9.08536 2.12292 9.74775 2.39254 10.3078 2.80461L11.0268 2.08559ZM7.02379 12.0211H12.803L12.4321 10.9085C12.133 10.0352 11.9894 9.12596 11.9894 8.21674V7.05545L12.9938 6.05112C12.9946 6.099 12.9948 6.14694 12.9944 6.19493V8.19282C12.9944 9.00633 13.126 9.80788 13.3772 10.5735L13.9634 12.3441L13.4849 13.0021H9.97959C9.97959 13.5285 9.76425 14.0429 9.39338 14.4138C9.02252 14.7847 8.50809 15 7.9817 15C7.45531 15 6.94088 14.7847 6.57002 14.4138C6.21281 14.0566 5.99988 13.5662 5.98468 13.0602L7.02379 12.0211ZM7.9817 14.019C8.24489 14.019 8.49613 13.9113 8.68754 13.7199C8.87896 13.5285 8.98663 13.2773 8.97466 13.0141H6.97677C6.97677 13.2773 7.08444 13.5285 7.27586 13.7199C7.46727 13.9113 7.7185 14.019 7.9817 14.019Z",
                fill_rule: "evenodd",
            }
            rect {
                height: "1",
                transform: "rotate(-45 1 14.5)",
                width: "20",
                x: "1",
                y: "14.5",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsBell;
impl IconShape for VsBell {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.377 10.573a7.63 7.63 0 0 1-.383-2.38V6.195a5.115 5.115 0 0 0-1.268-3.446 5.138 5.138 0 0 0-3.242-1.722c-.694-.072-1.4 0-2.07.227-.67.215-1.28.574-1.794 1.053a4.923 4.923 0 0 0-1.208 1.675 5.067 5.067 0 0 0-.431 2.022v2.2a7.61 7.61 0 0 1-.383 2.37L2 12.343l.479.658h3.505c0 .526.215 1.04.586 1.412.37.37.885.586 1.412.586.526 0 1.04-.215 1.411-.586s.587-.886.587-1.412h3.505l.478-.658-.586-1.77zm-4.69 3.147a.997.997 0 0 1-.705.299.997.997 0 0 1-.706-.3.997.997 0 0 1-.3-.705h1.999a.939.939 0 0 1-.287.706zm-5.515-1.71l.371-1.114a8.633 8.633 0 0 0 .443-2.691V6.004c0-.563.12-1.113.347-1.616.227-.514.55-.969.969-1.34.419-.382.91-.67 1.436-.837.538-.18 1.1-.24 1.65-.18a4.147 4.147 0 0 1 2.597 1.4 4.133 4.133 0 0 1 1.004 2.776v2.01c0 .909.144 1.818.443 2.691l.371 1.113h-9.63v-.012z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsBlank;
impl IconShape for VsBlank {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {

        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsBold;
impl IconShape for VsBold {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 13V3h3.362c1.116 0 1.954.224 2.515.673.565.449.848 1.113.848 1.992 0 .467-.137.881-.41 1.243-.273.357-.645.634-1.116.831.556.151.993.44 1.314.865.325.422.487.925.487 1.511 0 .898-.299 1.603-.897 2.116-.598.513-1.443.769-2.536.769H5zm1.356-4.677v3.599h2.24c.63 0 1.127-.158 1.49-.474.367-.32.55-.76.55-1.319 0-1.204-.673-1.806-2.02-1.806h-2.26zm0-1.058h2.049c.593 0 1.066-.144 1.42-.433.357-.288.536-.68.536-1.174 0-.55-.165-.948-.494-1.195-.33-.252-.831-.378-1.505-.378H6.356v3.18z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsBook;
impl IconShape for VsBook {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.5 2H9l-.35.15-.65.64-.65-.64L7 2H1.5l-.5.5v10l.5.5h5.29l.86.85h.7l.86-.85h5.29l.5-.5v-10l-.5-.5zm-7 10.32l-.18-.17L7 12H2V3h4.79l.74.74-.03 8.58zM14 12H9l-.35.15-.14.13V3.7l.7-.7H14v9zM6 5H3v1h3V5zm0 4H3v1h3V9zM3 7h3v1H3V7zm10-2h-3v1h3V5zm-3 2h3v1h-3V7zm0 2h3v1h-3V9z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsBookmark;
impl IconShape for VsBookmark {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12.5 1h-9l-.5.5v13l.872.335L8 10.247l4.128 4.588L13 14.5v-13l-.5-.5zM12 13.2L8.372 9.165h-.744L4 13.2V2h8v11.2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsBracketDot;
impl IconShape for VsBracketDot {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2.97184V2.98361H5.91083C5.71113 2.98361 5.5238 3.02427 5.34802 3.10513C5.17461 3.18275 5.02193 3.28942 4.89086 3.42437C4.76421 3.55475 4.66135 3.71034 4.58238 3.89205C4.50833 4.07152 4.47134 4.26019 4.47134 4.45902C4.47134 4.68725 4.4753 4.9134 4.48321 5.13749C4.49125 5.36105 4.49127 5.58262 4.48324 5.80219C4.47914 6.01973 4.46082 6.2333 4.42826 6.44285C4.39513 6.65175 4.33913 6.85263 4.26039 7.04464C4.18091 7.23843 4.07258 7.42254 3.93616 7.59702C3.82345 7.74119 3.68538 7.87538 3.52283 8C3.68538 8.12462 3.82345 8.25881 3.93616 8.40298C4.07258 8.57746 4.18091 8.76157 4.26039 8.95536C4.33921 9.14757 4.39513 9.35024 4.42823 9.56312C4.46084 9.76883 4.47914 9.98246 4.48324 10.2039C4.49127 10.4195 4.49125 10.6411 4.48321 10.8686C4.4753 11.0885 4.47134 11.3127 4.47134 11.541C4.47134 11.744 4.50838 11.9346 4.58223 12.1137C4.66104 12.2911 4.76386 12.4469 4.89086 12.5818C5.02194 12.7126 5.17396 12.8191 5.34763 12.9008C5.52346 12.9777 5.71095 13.0164 5.91083 13.0164H6V13.2V14H5.91083C5.59743 14 5.29407 13.9384 5.00128 13.8153C4.70818 13.692 4.44942 13.5153 4.22578 13.285C4.00311 13.0558 3.83793 12.805 3.73283 12.5323L3.73232 12.531C3.63387 12.265 3.56819 11.9903 3.53535 11.7072L3.53516 11.7055C3.50677 11.4215 3.4987 11.1316 3.51084 10.8357C3.52272 10.5462 3.52866 10.2567 3.52866 9.96721C3.52866 9.76883 3.48986 9.58047 3.41201 9.40108L3.41129 9.39942C3.33659 9.21871 3.23428 9.0637 3.10412 8.93352L3.10221 8.93161C2.97577 8.79762 2.82457 8.69157 2.64742 8.61396L2.64601 8.61334C2.47001 8.53238 2.28465 8.4918 2.08917 8.4918H2V8.4V7.6V7.5082H2.08917C2.28497 7.5082 2.4706 7.46954 2.64672 7.3925C2.82466 7.31055 2.97644 7.20405 3.10317 7.07359C3.23423 6.93866 3.33687 6.78296 3.4116 6.60601L3.412 6.60507C3.48974 6.42594 3.52866 6.23556 3.52866 6.03279C3.52866 5.74329 3.52272 5.45379 3.51084 5.16428C3.4987 4.86844 3.50678 4.5805 3.53519 4.30053L3.53533 4.29917C3.56814 4.01201 3.63382 3.7352 3.73233 3.46898L3.73282 3.46766C3.83792 3.19498 4.00311 2.94422 4.22578 2.71498C4.44942 2.48474 4.70818 2.30798 5.00128 2.18473C5.29407 2.06161 5.59743 2 5.91083 2H6V2.97184ZM13.9232 8.4918H14V8.4V7.6V7.5082H13.9108C13.7153 7.5082 13.53 7.46762 13.354 7.38666L13.3526 7.38604C13.1754 7.30844 13.0242 7.20238 12.8978 7.06839L12.8959 7.06648C12.7657 6.9363 12.6634 6.78129 12.5887 6.60058L12.588 6.59892C12.5101 6.41953 12.4713 6.23117 12.4713 6.03279C12.4713 5.74329 12.4773 5.45379 12.4892 5.16428C12.5013 4.86842 12.4932 4.57848 12.4648 4.29454L12.4646 4.29285C12.4318 4.00971 12.3661 3.73502 12.2677 3.46897L12.2672 3.46766C12.1621 3.19499 11.9969 2.94422 11.7742 2.71498C11.5506 2.48474 11.2918 2.30798 10.9987 2.18473C10.7059 2.06161 10.4026 2 10.0892 2H10V2.8V2.98361H10.0892C10.2891 2.98361 10.4765 3.0223 10.6524 3.09917C10.826 3.18092 10.9781 3.28736 11.1091 3.41823C11.2361 3.55305 11.339 3.70889 11.4178 3.88628C11.4916 4.0654 11.5287 4.25596 11.5287 4.45902C11.5287 4.68727 11.5247 4.91145 11.5168 5.13142C11.5088 5.35894 11.5087 5.58049 11.5168 5.79605C11.5209 6.01754 11.5392 6.23117 11.5718 6.43688C11.6049 6.64976 11.6608 6.85243 11.7396 7.04464C11.8191 7.23843 11.9274 7.42254 12.0638 7.59702C12.1765 7.74119 12.3146 7.87538 12.4772 8C12.4666 8.00814 12.456 8.01632 12.4456 8.02455C12.9764 8.08338 13.4758 8.24605 13.9232 8.4918Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M12 9C11.4067 9 10.8266 9.17595 10.3333 9.50559C9.83994 9.83524 9.45543 10.3038 9.22836 10.8519C9.0013 11.4001 8.94189 12.0033 9.05765 12.5853C9.1734 13.1672 9.45912 13.7018 9.87868 14.1213C10.2982 14.5409 10.8328 14.8266 11.4147 14.9424C11.9967 15.0581 12.5999 14.9987 13.1481 14.7716C13.6962 14.5446 14.1648 14.1601 14.4944 13.6667C14.8241 13.1734 15 12.5933 15 12C14.999 11.2047 14.6826 10.4422 14.1202 9.87976C13.5578 9.31736 12.7954 9.00098 12 9Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsBracketError;
impl IconShape for VsBracketError {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2.97184V2.98361H5.91083C5.71113 2.98361 5.5238 3.02427 5.34802 3.10513C5.17461 3.18275 5.02193 3.28942 4.89086 3.42437C4.76421 3.55475 4.66135 3.71034 4.58238 3.89205C4.50833 4.07152 4.47134 4.26019 4.47134 4.45902C4.47134 4.68725 4.4753 4.9134 4.48321 5.13749C4.49125 5.36105 4.49127 5.58262 4.48324 5.80219C4.47914 6.01973 4.46082 6.2333 4.42826 6.44285C4.39513 6.65175 4.33913 6.85263 4.26039 7.04464C4.18091 7.23843 4.07258 7.42254 3.93616 7.59702C3.82345 7.74119 3.68538 7.87538 3.52283 8C3.68538 8.12462 3.82345 8.25881 3.93616 8.40298C4.07258 8.57746 4.18091 8.76157 4.26039 8.95536C4.33921 9.14757 4.39513 9.35024 4.42823 9.56312C4.46084 9.76883 4.47914 9.98246 4.48324 10.2039C4.49127 10.4195 4.49125 10.6411 4.48321 10.8686C4.4753 11.0885 4.47134 11.3127 4.47134 11.541C4.47134 11.744 4.50838 11.9346 4.58223 12.1137C4.66104 12.2911 4.76386 12.4469 4.89086 12.5818C5.02194 12.7126 5.17396 12.8191 5.34763 12.9008C5.52346 12.9777 5.71095 13.0164 5.91083 13.0164H6V13.2V14H5.91083C5.59743 14 5.29407 13.9384 5.00128 13.8153C4.70818 13.692 4.44942 13.5153 4.22578 13.285C4.00311 13.0558 3.83793 12.805 3.73283 12.5323L3.73232 12.531C3.63387 12.265 3.56819 11.9903 3.53535 11.7072L3.53516 11.7055C3.50677 11.4215 3.4987 11.1316 3.51084 10.8357C3.52272 10.5462 3.52866 10.2567 3.52866 9.96721C3.52866 9.76883 3.48986 9.58047 3.41201 9.40108L3.41129 9.39942C3.33659 9.21871 3.23428 9.0637 3.10412 8.93352L3.10221 8.93161C2.97577 8.79762 2.82457 8.69157 2.64742 8.61396L2.64601 8.61334C2.47001 8.53238 2.28465 8.4918 2.08917 8.4918H2V8.4V7.6V7.5082H2.08917C2.28497 7.5082 2.4706 7.46954 2.64672 7.3925C2.82466 7.31055 2.97644 7.20405 3.10317 7.07359C3.23423 6.93866 3.33687 6.78296 3.4116 6.60601L3.412 6.60507C3.48974 6.42594 3.52866 6.23556 3.52866 6.03279C3.52866 5.74329 3.52272 5.45379 3.51084 5.16428C3.4987 4.86844 3.50678 4.5805 3.53519 4.30053L3.53533 4.29917C3.56814 4.01201 3.63382 3.7352 3.73233 3.46898L3.73282 3.46766C3.83792 3.19498 4.00311 2.94422 4.22578 2.71498C4.44942 2.48474 4.70818 2.30798 5.00128 2.18473C5.29407 2.06161 5.59743 2 5.91083 2H6V2.97184ZM13.9232 8.4918H14V8.4V7.6V7.5082H13.9108C13.7153 7.5082 13.53 7.46762 13.354 7.38666L13.3526 7.38604C13.1754 7.30844 13.0242 7.20238 12.8978 7.06839L12.8959 7.06648C12.7657 6.9363 12.6634 6.78129 12.5887 6.60058L12.588 6.59892C12.5101 6.41953 12.4713 6.23117 12.4713 6.03279C12.4713 5.74329 12.4773 5.45379 12.4892 5.16428C12.5013 4.86842 12.4932 4.57848 12.4648 4.29454L12.4646 4.29285C12.4318 4.00971 12.3661 3.73502 12.2677 3.46897L12.2672 3.46766C12.1621 3.19499 11.9969 2.94422 11.7742 2.71498C11.5506 2.48474 11.2918 2.30798 10.9987 2.18473C10.7059 2.06161 10.4026 2 10.0892 2H10V2.8V2.98361H10.0892C10.2891 2.98361 10.4765 3.0223 10.6524 3.09917C10.826 3.18092 10.9781 3.28736 11.1091 3.41823C11.2361 3.55305 11.339 3.70889 11.4178 3.88628C11.4916 4.0654 11.5287 4.25596 11.5287 4.45902C11.5287 4.68727 11.5247 4.91145 11.5168 5.13142C11.5088 5.35894 11.5087 5.58049 11.5168 5.79605C11.5209 6.01754 11.5392 6.23117 11.5718 6.43688C11.6049 6.64976 11.6608 6.85243 11.7396 7.04464C11.8191 7.23843 11.9274 7.42254 12.0638 7.59702C12.1765 7.74119 12.3146 7.87538 12.4772 8L12.4456 8.02455C12.9764 8.08338 13.4758 8.24605 13.9232 8.4918Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M10.3333 9.50559C10.8266 9.17595 11.4067 9 12 9C12.7954 9.00098 13.5578 9.31736 14.1202 9.87976C14.6826 10.4422 14.999 11.2047 15 12C15 12.5933 14.8241 13.1734 14.4944 13.6667C14.1648 14.1601 13.6962 14.5446 13.1481 14.7716C12.5999 14.9987 11.9967 15.0581 11.4147 14.9424C10.8328 14.8266 10.2982 14.5409 9.87868 14.1213C9.45912 13.7018 9.1734 13.1672 9.05765 12.5853C8.94189 12.0033 9.0013 11.4001 9.22836 10.8519C9.45543 10.3038 9.83994 9.83524 10.3333 9.50559ZM13.1464 10.1464L12 11.2929L10.8536 10.1464L10.1465 10.8535L11.2929 12L10.1464 13.1464L10.8536 13.8536L12 12.7071L13.1465 13.8535L13.8536 13.1464L12.7071 12L13.8536 10.8536L13.1464 10.1464Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsBriefcase;
impl IconShape for VsBriefcase {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.5 4H11V2.5l-.5-.5h-5l-.5.5V4H1.5l-.5.5v8l.5.5h13l.5-.5v-8l-.5-.5zM6 3h4v1H6V3zm8 2v.76L10 8v-.5L9.51 7h-3L6 7.5V8L2 5.71V5h12zM9 8v1H7V8h2zm-7 4V6.86l4 2.29v.35l.5.5h3l.5-.5v-.31l4-2.28V12H2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsBroadcast;
impl IconShape for VsBroadcast {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.667 2.011A6 6 0 0 1 8 1a6.007 6.007 0 0 1 6 6 6 6 0 0 1-3.996 5.655v-.044c.016-.014.031-.03.046-.045a1.48 1.48 0 0 0 .434-1.046v-.137A5.042 5.042 0 0 0 12.19 4.2a5.04 5.04 0 1 0-6.69 7.176v.144a1.48 1.48 0 0 0 .48 1.09v.04A5.999 5.999 0 0 1 4.667 2.01z",
            }
            path {
                d: "M9.343 11.86a.48.48 0 0 1-.34.14v2.52a.48.48 0 0 1-.48.48H7.46c.011 0-.004-.004-.034-.012-.075-.02-.241-.064-.305-.129a.48.48 0 0 1-.141-.34V12a.48.48 0 0 1-.48-.48V9.5a1 1 0 0 1 1-1h.984a1 1 0 0 1 1 1v2.02a.48.48 0 0 1-.137.335l-.004.004z",
            }
            path {
                d: "M10.64 7c0 .525-.157 1.034-.445 1.465.183.302.289.656.289 1.035v.106a3.596 3.596 0 0 0 .06-5.15A3.6 3.6 0 1 0 5.5 9.59V9.5c0-.384.108-.743.296-1.047A2.64 2.64 0 1 1 10.64 7z",
            }
            path {
                d: "M9 7a1 1 0 1 1-2 0 1 1 0 0 1 2 0z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsBrowser;
impl IconShape for VsBrowser {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 1h13l.5.5v12l-.5.5h-13l-.5-.5v-12l.5-.5zM2 5v8h12V5H2zm0-1h12V2H2v2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsBug;
impl IconShape for VsBug {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.877 4.5v-.582a2.918 2.918 0 1 0-5.836 0V4.5h-.833L2.545 2.829l-.593.59 1.611 1.619-.019.049a8.03 8.03 0 0 0-.503 2.831c0 .196.007.39.02.58l.003.045H1v.836h2.169l.006.034c.172.941.504 1.802.954 2.531l.034.055L2.2 13.962l.592.592 1.871-1.872.058.066c.868.992 2.002 1.589 3.238 1.589 1.218 0 2.336-.579 3.199-1.544l.057-.064 1.91 1.92.593-.591-1.996-2.006.035-.056c.467-.74.81-1.619.986-2.583l.006-.034h2.171v-.836h-2.065l.003-.044a8.43 8.43 0 0 0 .02-.58 8.02 8.02 0 0 0-.517-2.866l-.019-.05 1.57-1.57-.592-.59L11.662 4.5h-.785zm-5 0v-.582a2.082 2.082 0 1 1 4.164 0V4.5H5.878zm5.697.837l.02.053c.283.753.447 1.61.447 2.528 0 1.61-.503 3.034-1.274 4.037-.77 1.001-1.771 1.545-2.808 1.545-1.036 0-2.037-.544-2.807-1.545-.772-1.003-1.275-2.427-1.275-4.037 0-.918.164-1.775.448-2.528l.02-.053h7.229z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCalendar;
impl IconShape for VsCalendar {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.5 2H13V1h-1v1H4V1H3v1H1.5l-.5.5v12l.5.5h13l.5-.5v-12l-.5-.5zM14 14H2V5h12v9zm0-10H2V3h12v1zM4 8H3v1h1V8zm-1 2h1v1H3v-1zm1 2H3v1h1v-1zm2-4h1v1H6V8zm1 2H6v1h1v-1zm-1 2h1v1H6v-1zm1-6H6v1h1V6zm2 2h1v1H9V8zm1 2H9v1h1v-1zm-1 2h1v1H9v-1zm1-6H9v1h1V6zm2 2h1v1h-1V8zm1 2h-1v1h1v-1zm-1-4h1v1h-1V6z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCallIncoming;
impl IconShape for VsCallIncoming {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.547 9.328a1.567 1.567 0 0 0-.594-.117 1.202 1.202 0 0 0-.555.101 2.762 2.762 0 0 0-.43.258 2.166 2.166 0 0 0-.359.328c-.104.12-.205.23-.304.329a2.409 2.409 0 0 1-.29.25.534.534 0 0 1-.695-.063 32.17 32.17 0 0 1-.328-.312c-.14-.136-.312-.3-.515-.493A61.776 61.776 0 0 1 7.844 9l-.68-.664a25.847 25.847 0 0 1-1.21-1.266 5.312 5.312 0 0 1-.391-.484c-.094-.135-.141-.234-.141-.297a.46.46 0 0 1 .101-.312c.073-.094.16-.19.258-.29.1-.098.209-.203.328-.312.12-.11.23-.227.329-.352.098-.125.182-.268.25-.43.067-.16.101-.343.101-.546a1.567 1.567 0 0 0-.453-1.102 7.604 7.604 0 0 1-.531-.578 6.487 6.487 0 0 0-.617-.64 4.207 4.207 0 0 0-.696-.516A1.46 1.46 0 0 0 3.742 1a1.567 1.567 0 0 0-1.101.453c-.271.271-.508.513-.711.727a4.006 4.006 0 0 0-.516.664 2.63 2.63 0 0 0-.312.765A4.39 4.39 0 0 0 1 4.625c0 .552.089 1.125.266 1.719.177.593.416 1.185.718 1.773.302.589.67 1.167 1.102 1.735.432.567.901 1.106 1.406 1.617.505.51 1.042.982 1.61 1.414.567.432 1.148.805 1.742 1.117.593.313 1.19.557 1.789.734a6.157 6.157 0 0 0 1.75.266 4.696 4.696 0 0 0 1.008-.11 2.59 2.59 0 0 0 .773-.312c.23-.14.45-.312.664-.515.214-.204.453-.438.719-.704A1.568 1.568 0 0 0 15 12.257a2.009 2.009 0 0 0-.102-.515 1.674 1.674 0 0 0-.257-.484 7.24 7.24 0 0 0-.368-.445 5.381 5.381 0 0 0-.421-.422 91.549 91.549 0 0 0-.43-.383 8.277 8.277 0 0 1-.367-.344 1.516 1.516 0 0 0-.508-.336zm-.367 4.586a3.13 3.13 0 0 1-.797.086 5.526 5.526 0 0 1-1.516-.242 8.362 8.362 0 0 1-1.586-.664 13.205 13.205 0 0 1-3.047-2.297 17.15 17.15 0 0 1-1.289-1.461 10.502 10.502 0 0 1-1.03-1.578 9.12 9.12 0 0 1-.673-1.61A5.308 5.308 0 0 1 2 4.602a3.34 3.34 0 0 1 .094-.79c.057-.218.143-.414.258-.585.114-.172.255-.339.421-.5.167-.162.357-.35.57-.563a.542.542 0 0 1 .4-.164c.062-.005.158.036.288.125.13.089.271.195.422.32a7.058 7.058 0 0 1 .899.899c.125.15.229.289.312.414.083.125.125.221.125.289a.429.429 0 0 1-.101.312c-.073.084-.16.18-.258.29-.1.109-.209.213-.328.312-.12.099-.23.216-.329.351a2.266 2.266 0 0 0-.25.438 1.345 1.345 0 0 0-.101.54c.005.213.047.413.125.6.078.188.19.355.336.5l3.726 3.727a1.527 1.527 0 0 0 1.102.46 1.2 1.2 0 0 0 .547-.1 2.414 2.414 0 0 0 .789-.586c.11-.12.21-.23.305-.329.093-.098.19-.182.289-.25a.545.545 0 0 1 .312-.101c.073 0 .172.042.297.125.125.083.263.19.414.32.151.13.307.274.469.43.161.156.305.312.43.469.124.156.229.297.312.422.083.125.125.22.125.289a.533.533 0 0 1-.164.39c-.224.219-.414.41-.57.57a3.159 3.159 0 0 1-.5.422 1.93 1.93 0 0 1-.586.266zM15 1.704l-4.64 4.648h3.288v1h-5v-5h1V5.64L14.297 1l.703.703z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCallOutgoing;
impl IconShape for VsCallOutgoing {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.648 6.648L13.29 2H10V1h5v5h-1V2.71L9.352 7.353l-.704-.704zm3.305 2.563a1.567 1.567 0 0 1 1.102.453c.11.11.232.224.367.344l.43.383c.15.135.291.276.421.421.13.146.253.295.368.446.114.15.2.312.257.484.058.172.092.344.102.516a1.568 1.568 0 0 1-.453 1.101c-.266.266-.505.5-.719.704a4.006 4.006 0 0 1-.664.515c-.23.14-.487.245-.773.313a4.696 4.696 0 0 1-1.008.109 6.157 6.157 0 0 1-1.75-.266A9.819 9.819 0 0 1 7.843 14a12.445 12.445 0 0 1-1.741-1.117 15.329 15.329 0 0 1-1.61-1.414c-.505-.51-.974-1.05-1.406-1.617a11.64 11.64 0 0 1-1.102-1.735 10.38 10.38 0 0 1-.718-1.773A6.005 6.005 0 0 1 1 4.625c0-.396.034-.734.102-1.016a2.63 2.63 0 0 1 .312-.765c.14-.23.313-.45.516-.664.203-.214.44-.456.71-.727A1.567 1.567 0 0 1 3.743 1c.26 0 .51.07.75.21.24.142.472.313.696.517.223.203.43.416.617.64.187.224.364.417.53.578a1.567 1.567 0 0 1 .453 1.102 1.4 1.4 0 0 1-.1.547 1.824 1.824 0 0 1-.25.43 2.983 2.983 0 0 1-.329.351c-.12.11-.229.214-.328.313a3.128 3.128 0 0 0-.258.289.46.46 0 0 0-.101.312c0 .063.047.162.14.297a5.3 5.3 0 0 0 .391.484 24.386 24.386 0 0 0 1.211 1.266c.234.23.461.45.68.664.218.214.43.417.633.61.203.192.375.356.515.492.14.135.25.24.328.312a.534.534 0 0 0 .696.063c.093-.068.19-.152.289-.25.099-.1.2-.209.304-.329.104-.12.224-.229.36-.328.135-.099.278-.185.43-.258a1.21 1.21 0 0 1 .554-.101zM11.383 14c.318 0 .583-.029.797-.086a1.93 1.93 0 0 0 .586-.266c.177-.12.343-.26.5-.421.156-.162.346-.352.57-.57.11-.11.164-.24.164-.391 0-.068-.042-.164-.125-.29a6.122 6.122 0 0 0-.313-.421 5.01 5.01 0 0 0-.43-.47c-.16-.155-.317-.299-.468-.429a4.322 4.322 0 0 0-.414-.32c-.125-.083-.224-.125-.297-.125a.545.545 0 0 0-.312.101 1.801 1.801 0 0 0-.29.25c-.093.1-.195.209-.304.329-.11.12-.23.229-.36.328-.13.099-.273.185-.43.258a1.208 1.208 0 0 1-.546.101 1.527 1.527 0 0 1-1.102-.46L4.883 7.39a1.537 1.537 0 0 1-.336-.5 1.655 1.655 0 0 1-.125-.602c0-.203.034-.383.101-.539.068-.156.151-.302.25-.438.1-.135.209-.252.329-.351.12-.099.229-.203.328-.313.099-.109.185-.205.258-.289a.429.429 0 0 0 .101-.312c0-.068-.042-.164-.125-.29a5.085 5.085 0 0 0-.312-.413 6.791 6.791 0 0 0-.43-.469 6.787 6.787 0 0 0-.469-.43 5.674 5.674 0 0 0-.422-.32c-.13-.089-.226-.13-.289-.125a.542.542 0 0 0-.398.164 65.24 65.24 0 0 1-.57.563 3.073 3.073 0 0 0-.422.5 1.9 1.9 0 0 0-.258.586A3.377 3.377 0 0 0 2 4.601c0 .5.08 1.015.242 1.546a9.12 9.12 0 0 0 .672 1.61c.287.541.63 1.068 1.031 1.578.401.51.831.997 1.29 1.46a13.205 13.205 0 0 0 3.046 2.298 8.37 8.37 0 0 0 1.586.664 5.526 5.526 0 0 0 1.516.242z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCaseSensitive;
impl IconShape for VsCaseSensitive {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.85352 11.7021H7.85449L7.03809 9.54297H3.77246L3.00439 11.7021H2L4.9541 4H5.88867L8.85352 11.7021ZM6.74268 8.73193L5.53418 5.4502C5.49479 5.34277 5.4554 5.1709 5.41602 4.93457H5.39453C5.35872 5.15299 5.31755 5.32487 5.271 5.4502L4.07324 8.73193H6.74268Z",
            }
            path {
                d: "M13.756 11.7021H12.8752V10.8428H12.8537C12.4706 11.5016 11.9066 11.8311 11.1618 11.8311C10.6139 11.8311 10.1843 11.686 9.87273 11.396C9.56479 11.106 9.41082 10.721 9.41082 10.2412C9.41082 9.21354 10.016 8.61556 11.2262 8.44727L12.8752 8.21631C12.8752 7.28174 12.4974 6.81445 11.7419 6.81445C11.0794 6.81445 10.4815 7.04004 9.94793 7.49121V6.58887C10.4886 6.24512 11.1117 6.07324 11.8171 6.07324C13.1097 6.07324 13.756 6.75716 13.756 8.125V11.7021ZM12.8752 8.91992L11.5485 9.10254C11.1403 9.15983 10.8324 9.26188 10.6247 9.40869C10.417 9.55192 10.3132 9.80794 10.3132 10.1768C10.3132 10.4453 10.4081 10.6655 10.5978 10.8374C10.7912 11.0057 11.0472 11.0898 11.3659 11.0898C11.8027 11.0898 12.1626 10.9377 12.4455 10.6333C12.7319 10.3254 12.8752 9.93685 12.8752 9.46777V8.91992Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCheckAll;
impl IconShape for VsCheckAll {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M15.62 3.596L7.815 12.81l-.728-.033L4 8.382l.754-.53 2.744 3.907L14.917 3l.703.596z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M7.234 8.774l4.386-5.178L10.917 3l-4.23 4.994.547.78zm-1.55.403l.548.78-.547-.78zm-1.617 1.91l.547.78-.799.943-.728-.033L0 8.382l.754-.53 2.744 3.907.57-.672z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCheck;
impl IconShape for VsCheck {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.431 3.323l-8.47 10-.79-.036-3.35-4.77.818-.574 2.978 4.24 8.051-9.506.764.646z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsChecklist;
impl IconShape for VsChecklist {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3.75 4.48h-.71L2 3.43l.71-.7.69.68L4.81 2l.71.71-1.77 1.77zM6.99 3h8v1h-8V3zm0 3h8v1h-8V6zm8 3h-8v1h8V9zm-8 3h8v1h-8v-1zM3.04 7.48h.71l1.77-1.77-.71-.7L3.4 6.42l-.69-.69-.71.71 1.04 1.04zm.71 3.01h-.71L2 9.45l.71-.71.69.69 1.41-1.42.71.71-1.77 1.77zm-.71 3.01h.71l1.77-1.77-.71-.71-1.41 1.42-.69-.69-.71.7 1.04 1.05z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsChevronDown;
impl IconShape for VsChevronDown {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.976 10.072l4.357-4.357.62.618L8.284 11h-.618L3 6.333l.619-.618 4.357 4.357z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsChevronLeft;
impl IconShape for VsChevronLeft {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.928 7.976l4.357 4.357-.618.62L5 8.284v-.618L9.667 3l.618.619-4.357 4.357z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsChevronRight;
impl IconShape for VsChevronRight {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.072 8.024L5.715 3.667l.618-.62L11 7.716v.618L6.333 13l-.618-.619 4.357-4.357z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsChevronUp;
impl IconShape for VsChevronUp {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.024 5.928l-4.357 4.357-.62-.618L7.716 5h.618L13 9.667l-.619.618-4.357-4.357z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsChip;
impl IconShape for VsChip {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 1H6V3H7V1H8V3H9V1H10V3H11L12 4V5L14 5V6H12V7L14 7V8H12V9L14 9V10H12V11L11 12H10V14H9V12H8V14H7V12H6V14H5V12H4L3 11L3 10H1V9L3 9L3 8H1V7L3 7L3 6H1V5L3 5L3 4L4 3H5L5 1ZM4 11H11V4L4 4L4 11Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsChromeClose;
impl IconShape for VsChromeClose {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.116 8l-4.558 4.558.884.884L8 8.884l4.558 4.558.884-.884L8.884 8l4.558-4.558-.884-.884L8 7.116 3.442 2.558l-.884.884L7.116 8z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsChromeMaximize;
impl IconShape for VsChromeMaximize {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 3v10h10V3H3zm9 9H4V4h8v8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsChromeMinimize;
impl IconShape for VsChromeMinimize {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 8v1H3V8h11z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsChromeRestore;
impl IconShape for VsChromeRestore {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 5v9h9V5H3zm8 8H4V6h7v7z",
            }
            path {
                clip_rule: "evenodd",
                d: "M5 5h1V4h7v7h-1v1h2V3H5v2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCircleFilled;
impl IconShape for VsCircleFilled {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 4c.367 0 .721.048 1.063.145a3.943 3.943 0 0 1 1.762 1.031 3.944 3.944 0 0 1 1.03 1.762c.097.34.145.695.145 1.062 0 .367-.048.721-.145 1.063a3.94 3.94 0 0 1-1.03 1.765 4.017 4.017 0 0 1-1.762 1.031C8.72 11.953 8.367 12 8 12s-.721-.047-1.063-.14a4.056 4.056 0 0 1-1.765-1.032A4.055 4.055 0 0 1 4.14 9.062 3.992 3.992 0 0 1 4 8c0-.367.047-.721.14-1.063a4.02 4.02 0 0 1 .407-.953A4.089 4.089 0 0 1 5.98 4.546a3.94 3.94 0 0 1 .957-.401A3.89 3.89 0 0 1 8 4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCircleLargeFilled;
impl IconShape for VsCircleLargeFilled {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 1a6.8 6.8 0 0 1 1.86.253 6.899 6.899 0 0 1 3.083 1.805 6.903 6.903 0 0 1 1.804 3.083C14.916 6.738 15 7.357 15 8s-.084 1.262-.253 1.86a6.9 6.9 0 0 1-.704 1.674 7.157 7.157 0 0 1-2.516 2.509 6.966 6.966 0 0 1-1.668.71A6.984 6.984 0 0 1 8 15a6.984 6.984 0 0 1-1.86-.246 7.098 7.098 0 0 1-1.674-.711 7.3 7.3 0 0 1-1.415-1.094 7.295 7.295 0 0 1-1.094-1.415 7.098 7.098 0 0 1-.71-1.675A6.985 6.985 0 0 1 1 8c0-.643.082-1.262.246-1.86a6.968 6.968 0 0 1 .711-1.667 7.156 7.156 0 0 1 2.509-2.516 6.895 6.895 0 0 1 1.675-.704A6.808 6.808 0 0 1 8 1z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCircleLarge;
impl IconShape for VsCircleLarge {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.588 2.215A5.808 5.808 0 0 0 8 2c-.554 0-1.082.073-1.588.215l-.006.002c-.514.141-.99.342-1.432.601A6.156 6.156 0 0 0 2.82 4.98l-.002.004A5.967 5.967 0 0 0 2.21 6.41 5.986 5.986 0 0 0 2 8c0 .555.07 1.085.21 1.591a6.05 6.05 0 0 0 1.548 2.651c.37.365.774.677 1.216.94a6.1 6.1 0 0 0 1.435.609A6.02 6.02 0 0 0 8 14c.555 0 1.085-.07 1.591-.21.515-.145.99-.348 1.426-.607l.004-.002a6.16 6.16 0 0 0 2.161-2.155 5.85 5.85 0 0 0 .6-1.432l.003-.006A5.807 5.807 0 0 0 14 8c0-.554-.072-1.082-.215-1.588l-.002-.006a5.772 5.772 0 0 0-.6-1.423l-.002-.004a5.9 5.9 0 0 0-.942-1.21l-.008-.008a5.902 5.902 0 0 0-1.21-.942l-.004-.002a5.772 5.772 0 0 0-1.423-.6l-.006-.002zm4.455 9.32a7.157 7.157 0 0 1-2.516 2.508 6.966 6.966 0 0 1-1.668.71A6.984 6.984 0 0 1 8 15a6.984 6.984 0 0 1-1.86-.246 7.098 7.098 0 0 1-1.674-.711 7.3 7.3 0 0 1-1.415-1.094 7.295 7.295 0 0 1-1.094-1.415 7.098 7.098 0 0 1-.71-1.675A6.985 6.985 0 0 1 1 8c0-.643.082-1.262.246-1.86a6.968 6.968 0 0 1 .711-1.667 7.156 7.156 0 0 1 2.509-2.516 6.895 6.895 0 0 1 1.675-.704A6.808 6.808 0 0 1 8 1a6.8 6.8 0 0 1 1.86.253 6.899 6.899 0 0 1 3.083 1.805 6.903 6.903 0 0 1 1.804 3.083C14.916 6.738 15 7.357 15 8s-.084 1.262-.253 1.86a6.9 6.9 0 0 1-.704 1.674z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCircleSlash;
impl IconShape for VsCircleSlash {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 1a7 7 0 1 1-7 7 7.008 7.008 0 0 1 7-7zM2 8c0 1.418.504 2.79 1.423 3.87l8.447-8.447A5.993 5.993 0 0 0 2 8zm12 0c0-1.418-.504-2.79-1.423-3.87L4.13 12.577A5.993 5.993 0 0 0 14 8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCircleSmallFilled;
impl IconShape for VsCircleSmallFilled {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 8a2 2 0 1 1-4 0 2 2 0 0 1 4 0z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCircleSmall;
impl IconShape for VsCircleSmall {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.83185 8.55551C8.72192 8.72004 8.56569 8.84825 8.38305 8.9239C8.20039 8.99956 7.99936 9.01936 7.80531 8.98076C7.61133 8.94218 7.43313 8.84693 7.29328 8.70708C7.15343 8.56723 7.05818 8.38903 7.01959 8.19505C6.98099 8.001 7.0008 7.79996 7.07646 7.6173C7.15211 7.43466 7.28032 7.27844 7.44484 7.16851C7.60928 7.05863 7.80256 7 8.00036 7C8.26552 7 8.51986 7.10535 8.70743 7.29292C8.89501 7.4805 9.00036 7.73484 9.00036 8C9.00036 8.19779 8.94172 8.39108 8.83185 8.55551ZM9.66332 9.11108C9.88308 8.78219 10.0004 8.39556 10.0004 8C10.0004 7.46957 9.78961 6.96089 9.41454 6.58582C9.03947 6.21074 8.53079 6 8.00036 6C7.60479 6 7.21817 6.11727 6.88927 6.33704C6.56037 6.5568 6.30395 6.86917 6.15258 7.23462C6.0012 7.60007 5.96164 8.00217 6.03881 8.39014C6.11598 8.7781 6.30647 9.13448 6.58617 9.41418C6.86588 9.69389 7.22226 9.88438 7.61022 9.96155C7.99818 10.0387 8.40028 9.99915 8.76574 9.84778C9.13119 9.6964 9.44356 9.43998 9.66332 9.11108Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCircle;
impl IconShape for VsCircle {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8 12a4 4 0 1 0 0-8 4 4 0 0 0 0 8zm2.61-4a2.61 2.61 0 1 1-5.22 0 2.61 2.61 0 0 1 5.22 0zM8 5.246z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCircuitBoard;
impl IconShape for VsCircuitBoard {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.5 1h-13l-.5.5v13l.5.5h13l.5-.5v-13l-.5-.5zM14 14H5v-2h2.3c.3.6 1 1 1.7 1 1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2H4v3H2V2h2v2.3c-.6.3-1 1-1 1.7 0 1.1.9 2 2 2s2-.9 2-2h2c0 1.1.9 2 2 2s2-.9 2-2-.9-2-2-2c-.7 0-1.4.4-1.7 1H6.7c-.3-.6-1-1-1.7-1V2h9v12zm-6-3c0-.6.4-1 1-1s1 .4 1 1-.4 1-1 1-1-.4-1-1zM5 5c.6 0 1 .4 1 1s-.4 1-1 1-1-.4-1-1 .4-1 1-1zm6 0c.6 0 1 .4 1 1s-.4 1-1 1-1-.4-1-1 .4-1 1-1z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsClearAll;
impl IconShape for VsClearAll {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 12.6l.7.7 1.6-1.6 1.6 1.6.8-.7L13 11l1.7-1.6-.8-.8-1.6 1.7-1.6-1.7-.7.8 1.6 1.6-1.6 1.6zM1 4h14V3H1v1zm0 3h14V6H1v1zm8 2.5V9H1v1h8v-.5zM9 13v-1H1v1h8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsClippy;
impl IconShape for VsClippy {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7 13.992H4v-9h8v2h1v-2.5l-.5-.5H11v-1h-1a2 2 0 0 0-4 0H4.94v1H3.5l-.5.5v10l.5.5H7v-1zm0-11.2a1 1 0 0 1 .8-.8 1 1 0 0 1 .58.06.94.94 0 0 1 .45.36 1 1 0 1 1-1.75.94 1 1 0 0 1-.08-.56zm7.08 9.46L13 13.342v-5.35h-1v5.34l-1.08-1.08-.71.71 1.94 1.93h.71l1.93-1.93-.71-.71zm-5.92-4.16h.71l1.93 1.93-.71.71-1.08-1.08v5.34h-1v-5.35l-1.08 1.09-.71-.71 1.94-1.93z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCloseAll;
impl IconShape for VsCloseAll {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.621 8.086l-.707-.707L6.5 8.793 5.086 7.379l-.707.707L5.793 9.5l-1.414 1.414.707.707L6.5 10.207l1.414 1.414.707-.707L7.207 9.5l1.414-1.414z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M5 3l1-1h7l1 1v7l-1 1h-2v2l-1 1H3l-1-1V6l1-1h2V3zm1 2h4l1 1v4h2V3H6v2zm4 1H3v7h7V6z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsClose;
impl IconShape for VsClose {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8 8.707l3.646 3.647.708-.707L8.707 8l3.647-3.646-.707-.708L8 7.293 4.354 3.646l-.707.708L7.293 8l-3.646 3.646.707.708L8 8.707z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCloudDownload;
impl IconShape for VsCloudDownload {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.957 6h.05a2.99 2.99 0 0 1 2.116.879 3.003 3.003 0 0 1 0 4.242 2.99 2.99 0 0 1-2.117.879v-1a2.002 2.002 0 0 0 0-4h-.914l-.123-.857a2.49 2.49 0 0 0-2.126-2.122A2.478 2.478 0 0 0 6.231 5.5l-.333.762-.809-.189A2.49 2.49 0 0 0 4.523 6c-.662 0-1.297.263-1.764.732A2.503 2.503 0 0 0 4.523 11h.498v1h-.498a3.486 3.486 0 0 1-2.628-1.16 3.502 3.502 0 0 1 1.958-5.78 3.462 3.462 0 0 1 1.468.04 3.486 3.486 0 0 1 3.657-2.06A3.479 3.479 0 0 1 11.957 6zm-5.25 5.121l1.314 1.314V7h.994v5.4l1.278-1.279.707.707-2.146 2.147h-.708L6 11.829l.707-.708z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCloudUpload;
impl IconShape for VsCloudUpload {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.956 6h.05a2.99 2.99 0 0 1 2.117.879 3.003 3.003 0 0 1 0 4.242 2.99 2.99 0 0 1-2.117.879h-1.995v-1h1.995a2.002 2.002 0 0 0 0-4h-.914l-.123-.857a2.49 2.49 0 0 0-2.126-2.122A2.478 2.478 0 0 0 6.23 5.5l-.333.762-.809-.189A2.49 2.49 0 0 0 4.523 6c-.662 0-1.297.263-1.764.732A2.503 2.503 0 0 0 4.523 11h2.494v1H4.523a3.486 3.486 0 0 1-2.628-1.16 3.502 3.502 0 0 1-.4-4.137A3.497 3.497 0 0 1 3.853 5.06c.486-.09.987-.077 1.468.041a3.486 3.486 0 0 1 3.657-2.06A3.479 3.479 0 0 1 11.956 6zm-1.663 3.853L8.979 8.54v5.436h-.994v-5.4L6.707 9.854 6 9.146 8.146 7h.708L11 9.146l-.707.707z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCloud;
impl IconShape for VsCloud {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.957 6h.05a2.99 2.99 0 0 1 2.116.879 3.003 3.003 0 0 1 0 4.242 2.99 2.99 0 0 1-2.117.879v-.013L12 12H4.523a3.486 3.486 0 0 1-2.628-1.16 3.502 3.502 0 0 1 1.958-5.78 3.462 3.462 0 0 1 1.468.04 3.486 3.486 0 0 1 3.657-2.06A3.479 3.479 0 0 1 11.957 6zM5 11h7.01a1.994 1.994 0 0 0 1.992-2 2.002 2.002 0 0 0-1.996-2h-.914l-.123-.857a2.49 2.49 0 0 0-2.126-2.122A2.478 2.478 0 0 0 6.231 5.5l-.333.762-.809-.189A2.49 2.49 0 0 0 4.523 6c-.662 0-1.297.263-1.764.732A2.503 2.503 0 0 0 4.523 11H5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCodeOss;
impl IconShape for VsCodeOss {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 2H6V13H5V2Z",
            }
            path {
                d: "M7 3H11V4H7V3Z",
            }
            path {
                d: "M9 5H13V6H9V5Z",
            }
            path {
                d: "M9 7H13V8H9V7Z",
            }
            path {
                d: "M9 9H13V10H9V9Z",
            }
            path {
                d: "M7 11H11V12H7V11Z",
            }
            path {
                d: "M2.00012 1L1.00012 2V7H2.00012V2H14.0001V13H2.00012V7H1.00012V13L2.00012 14H14.0001L15.0001 13V2L14.0001 1H2.00012Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCodeReview;
impl IconShape for VsCodeReview {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.5 2H1.5L1 2.5V11.5L1.5 12H4V14.5L4.854 14.854L7.707 12H14.5L15 11.5V2.5L14.5 2ZM14 11H7.5L7.146 11.146L5 13.293V11.5L4.5 11H2V3H14V11Z",
            }
            path {
                d: "M7.079 5.205L5.262 7.033L7.078 8.853L6.371 9.56L4.2 7.386V6.679L6.37 4.5L7.079 5.205ZM9.7 4.505L9 5.214L10.826 7.033L8.995 8.853L9.7 9.562L11.889 7.389V6.68L9.7 4.505Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCode;
impl IconShape for VsCode {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.708 5.578L2.061 8.224l2.647 2.646-.708.708-3-3V7.87l3-3 .708.708zm7-.708L11 5.578l2.647 2.646L11 10.87l.708.708 3-3V7.87l-3-3zM4.908 13l.894.448 5-10L9.908 3l-5 10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCoffee;
impl IconShape for VsCoffee {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 1V1.5C3 1.96954 3.27449 2.20587 3.8 2.6L3.83977 2.62978C4.31392 2.98457 5 3.49793 5 4.5V5H4V4.5C4 4.03046 3.72551 3.79413 3.2 3.4L3.16023 3.37022C2.68608 3.01543 2 2.50207 2 1.5V1H3Z",
            }
            path {
                d: "M6 1V1.5C6 1.96954 6.27449 2.20587 6.8 2.6L6.83977 2.62978C7.31392 2.98457 8 3.49793 8 4.5V5H7V4.5C7 4.03046 6.72551 3.79413 6.2 3.4L6.16023 3.37022C5.68608 3.01543 5 2.50207 5 1.5V1H6Z",
            }
            path {
                d: "M9 1V1.5C9 1.96954 9.27449 2.20587 9.8 2.6L9.83977 2.62978C10.3139 2.98457 11 3.49793 11 4.5V5H10V4.5C10 4.03046 9.72551 3.79413 9.2 3.4L9.16023 3.37022C8.68608 3.01543 8 2.50207 8 1.5V1H9Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M2 7L3 6H13.5C14.8807 6 16 7.11929 16 8.5C16 9.88071 14.8807 11 13.5 11H12.874C12.4299 12.7252 10.8638 14 9 14H6C3.79086 14 2 12.2091 2 10V7ZM12 10V7H3V10C3 11.6569 4.34315 13 6 13H9C10.6569 13 12 11.6569 12 10ZM13 7V10H13.5C14.3284 10 15 9.32843 15 8.5C15 7.67157 14.3284 7 13.5 7H13Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCollapseAll;
impl IconShape for VsCollapseAll {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 9H4v1h5V9z",
            }
            path {
                clip_rule: "evenodd",
                d: "M5 3l1-1h7l1 1v7l-1 1h-2v2l-1 1H3l-1-1V6l1-1h2V3zm1 2h4l1 1v4h2V3H6v2zm4 1H3v7h7V6z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsColorMode;
impl IconShape for VsColorMode {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 1a7 7 0 1 0 0 14A7 7 0 0 0 8 1zm0 13V2a6 6 0 1 1 0 12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCombine;
impl IconShape for VsCombine {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 1l-.5.5v3l.5.5h3l.5-.5v-3L4.5 1h-3zM2 4V2h2v2H2zm-.5 2l-.5.5v3l.5.5h3l.5-.5v-3L4.5 6h-3zM2 9V7h2v2H2zm-1 2.5l.5-.5h3l.5.5v3l-.5.5h-3l-.5-.5v-3zm1 .5v2h2v-2H2zm10.5-7l-.5.5v6l.5.5h3l.5-.5v-6l-.5-.5h-3zM15 8h-2V6h2v2zm0 3h-2V9h2v2zM9.1 8H6v1h3.1l-1 1 .7.6 1.8-1.8v-.7L8.8 6.3l-.7.7 1 1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCommentDiscussion;
impl IconShape for VsCommentDiscussion {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 11.29l1-1v1.42l-1.15 1.14L3 12.5V10H1.5L1 9.5v-8l.5-.5h12l.5.5V6h-1V2H2v7h1.5l.5.5v1.79zM10.29 13l1.86 1.85.85-.35V13h1.5l.5-.5v-5l-.5-.5h-8l-.5.5v5l.5.5h3.79zm.21-1H7V8h7v4h-1.5l-.5.5v.79l-1.15-1.14-.35-.15z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCommentDraft;
impl IconShape for VsCommentDraft {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.5 2.00098H13V3.00098H14V4.00098H15V2.50098L14.5 2.00098ZM9 2.00098H11V3.00098H9V2.00098ZM5 2.00098H7V3.00098H5V2.00098ZM14 10.001V11.001V12.001H14.5L15 11.501V10.001H14ZM12 12.001V11.001H10V12.001H12ZM8 11.001H7.5L7.146 11.147L5 13.294V11.501L4.5 11.001H4V12.001V14.501L4.854 14.855L7.707 12.001H8V11.001ZM15 8.00098V6.00098H14V8.00098H15ZM2 11.001V10.001H1V11.501L1.5 12.001H2V11.001ZM2 8.00098V6.00098H1V8.00098H2ZM2 3.00098V4.00098H1V2.50098L1.5 2.00098H3V3.00098H2Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCommentUnresolved;
impl IconShape for VsCommentUnresolved {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 2H14.5L15 2.5V9.35418C14.714 9.03018 14.3764 8.75287 14 8.53513V3H2V11H4.5L5 11.5V13.293L7.146 11.146L7.5 11H8.12602C8.04375 11.3196 8 11.6547 8 12H7.707L4.854 14.854L4 14.5V12H1.5L1 11.5V2.5L1.5 2Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M12 9C11.4067 9 10.8266 9.17595 10.3333 9.50559C9.83994 9.83524 9.45543 10.3038 9.22836 10.8519C9.0013 11.4001 8.94189 12.0033 9.05765 12.5853C9.1734 13.1672 9.45912 13.7018 9.87868 14.1213C10.2982 14.5409 10.8328 14.8266 11.4147 14.9424C11.9967 15.0581 12.5999 14.9987 13.1481 14.7716C13.6962 14.5446 14.1648 14.1601 14.4944 13.6667C14.8241 13.1734 15 12.5933 15 12C14.999 11.2047 14.6826 10.4422 14.1202 9.87976C13.5578 9.31736 12.7954 9.00098 12 9Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsComment;
impl IconShape for VsComment {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.5 2h-13l-.5.5v9l.5.5H4v2.5l.854.354L7.707 12H14.5l.5-.5v-9l-.5-.5zm-.5 9H7.5l-.354.146L5 13.293V11.5l-.5-.5H2V3h12v8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCompassActive;
impl IconShape for VsCompassActive {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.10146 13.8991C8.90419 13.9357 8.70353 13.9627 8.49999 13.9795V13H7.49999V13.9795C4.57233 13.7379 2.24067 11.3945 2.0175 8.46167H3V7.46167H2.02382C2.28141 4.56475 4.59788 2.25996 7.49999 2.02054V3H8.49999V2.02054C11.4149 2.26101 13.739 4.5851 13.9795 7.5H13V8.5H13.9795C13.9627 8.70354 13.9357 8.90419 13.8991 9.10146C14.2338 9.17833 14.5524 9.29718 14.8492 9.45217C14.948 8.98368 15 8.49791 15 8C15 4.13401 11.866 1 8 1C4.13401 1 1 4.13401 1 8C1 11.866 4.13401 15 8 15C8.49791 15 8.98368 14.948 9.45217 14.8492C9.29718 14.5524 9.17833 14.2338 9.10146 13.8991ZM9.90369 10.4675L6.99115 9.00874L4.96667 4.96655L9.00885 6.99103L10.4676 9.90359C10.2614 10.0724 10.0725 10.2613 9.90369 10.4675ZM9.43542 9.4353L8.48073 7.51916L6.56458 6.56447L7.51927 8.48062L9.43542 9.4353Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M11.3333 10.5056C11.8266 10.1759 12.4067 10 13 10C13.7954 10.001 14.5578 10.3174 15.1202 10.8798C15.6826 11.4422 15.999 12.2046 16 13C16 13.5933 15.8241 14.1734 15.4944 14.6667C15.1648 15.1601 14.6962 15.5446 14.1481 15.7716C13.5999 15.9987 12.9967 16.0581 12.4147 15.9424C11.8328 15.8266 11.2982 15.5409 10.8787 15.1213C10.4591 14.7018 10.1734 14.1672 10.0576 13.5853C9.94189 13.0033 10.0013 12.4001 10.2284 11.8519C10.4554 11.3038 10.8399 10.8352 11.3333 10.5056ZM13.0315 14.3226L14.8213 11.9363L14.0213 11.3363L12.541 13.3099L11.6655 12.6095L11.0408 13.3903L12.3192 14.413L13.0315 14.3226Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCompassDot;
impl IconShape for VsCompassDot {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.10146 13.8991C8.90419 13.9357 8.70353 13.9627 8.49999 13.9795V13H7.49999V13.9795C4.57233 13.7379 2.24067 11.3945 2.0175 8.46167H3V7.46167H2.02382C2.28141 4.56475 4.59788 2.25996 7.49999 2.02054V3H8.49999V2.02054C11.4149 2.26101 13.739 4.5851 13.9795 7.5H13V8.5H13.9795C13.9627 8.70354 13.9357 8.90419 13.8991 9.10146C14.2338 9.17833 14.5524 9.29718 14.8492 9.45217C14.948 8.98368 15 8.49791 15 8C15 4.13401 11.866 1 8 1C4.13401 1 1 4.13401 1 8C1 11.866 4.13401 15 8 15C8.49791 15 8.98368 14.948 9.45217 14.8492C9.29718 14.5524 9.17833 14.2338 9.10146 13.8991ZM9.90369 10.4675L6.99115 9.00874L4.96667 4.96655L9.00885 6.99103L10.4676 9.90359C10.2614 10.0724 10.0725 10.2613 9.90369 10.4675ZM9.43542 9.4353L8.48073 7.51916L6.56458 6.56447L7.51927 8.48062L9.43542 9.4353Z",
                fill_rule: "evenodd",
            }
            circle {
                cx: "13",
                cy: "13",
                r: "3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCompass;
impl IconShape for VsCompass {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.00885 6.99103L11.0333 11.0332L6.99114 9.00874L4.96666 4.96655L9.00885 6.99103ZM9.43541 9.4353L8.48072 7.51916L6.56458 6.56447L7.51926 8.48062L9.43541 9.4353Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M13.9795 8.5C13.739 11.4149 11.4149 13.739 8.49999 13.9795V13H7.49999V13.9795C4.57233 13.7379 2.24067 11.3945 2.0175 8.46167H3V7.46167H2.02382C2.28141 4.56475 4.59788 2.25996 7.49999 2.02054V3H8.49999V2.02054C11.4149 2.26101 13.739 4.5851 13.9795 7.5H13V8.5H13.9795ZM8 15C11.866 15 15 11.866 15 8C15 4.13401 11.866 1 8 1C4.13401 1 1 4.13401 1 8C1 11.866 4.13401 15 8 15Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCopilotBlocked;
impl IconShape for VsCopilotBlocked {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.3538 5.11442C14.3374 5.10449 14.321 5.09464 14.3045 5.08486C13.8842 4.83592 13.4309 4.63682 12.9531 4.49557C12.9831 4.30622 13 4.07388 13 3.78747C13 3.02182 12.8774 2.54541 12.6302 2.23253C12.3967 1.93694 11.9369 1.64619 10.9172 1.53288C9.87297 1.41686 9.42883 1.62362 9.23578 1.83046C9.03194 2.04886 8.87725 2.50883 8.9942 3.44444C9.05193 3.9063 9.1586 4.27849 9.30982 4.57107C8.92263 4.70333 8.55303 4.87406 8.20587 5.07875C7.48331 5.50479 6.85713 6.07889 6.36945 6.75962C5.897 6.94529 5.35448 7.03747 4.74633 7.03747C4.0941 7.03747 3.50959 6.95586 3.01944 6.74622L2.99634 6.86173V11.1171C3.2611 11.3217 4.2814 11.8416 5.57292 12.1956C5.69505 12.552 5.8498 12.8935 6.03367 13.2163C6.17631 13.4668 6.33639 13.7058 6.51224 13.9319C2.65002 13.3955 -0.00366211 10.9215 -0.00366211 10.2401V8.36753C0.0657067 7.74371 0.662905 6.65848 1.5876 6.27319C1.60075 6.20302 1.6121 6.1305 1.62392 6.05491C1.65252 5.87212 1.68393 5.67138 1.75044 5.44254C1.54926 4.93459 1.49631 4.3595 1.49631 3.78747C1.49631 2.91675 1.62376 2.01816 2.18906 1.30264C2.76803 0.569822 3.68325 0.178752 4.91348 0.0420517C6.11928 -0.0919193 7.17514 0.0763117 7.8571 0.806982C7.90658 0.860002 7.95294 0.915202 7.99631 0.971602C8.03967 0.915202 8.08969 0.860002 8.13918 0.806982C8.82113 0.0763117 9.87699 -0.0919193 11.0828 0.0420517C12.313 0.178752 13.2282 0.569822 13.8072 1.30264C14.3725 2.01816 14.5 2.91675 14.5 3.78747C14.5 4.23941 14.4669 4.69327 14.3538 5.11442ZM6.76184 1.83046C6.56879 1.62362 6.12465 1.41686 5.08045 1.53288C4.06068 1.64619 3.6009 1.93694 3.36737 2.23253C3.12018 2.54541 2.99763 3.02182 2.99763 3.78747C2.99763 4.57978 3.12668 4.95823 3.30583 5.15788C3.46808 5.33869 3.82531 5.53747 4.74763 5.53747C5.6013 5.53747 6.08666 5.30186 6.38574 4.99654C6.70112 4.67458 6.9127 4.17023 7.00342 3.44444C7.12037 2.50883 6.96568 2.04886 6.76184 1.83046Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M14.432 13.432C12.6746 15.1893 9.82538 15.1893 8.06802 13.432C6.31066 11.6746 6.31066 8.82538 8.06802 7.06802C9.82538 5.31066 12.6746 5.31066 14.432 7.06802C16.1893 8.82538 16.1893 11.6746 14.432 13.432ZM13.25 11C13.6642 11 14 10.6642 14 10.25C14 9.8358 13.6642 9.5 13.25 9.5H9.25C8.83579 9.5 8.5 9.8358 8.5 10.25C8.5 10.6642 8.83579 11 9.25 11H13.25Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCopilotLarge;
impl IconShape for VsCopilotLarge {
    fn view_box(&self) -> &str {
        "0 0 48 48"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M47.801 34.003c-1.72 2.988-11.706 10.037-23.82 10.037S1.881 36.991.161 34.003a1.309 1.309 0 0 1-.161-.57v-5.615c.012-.17.047-.338.11-.498.744-1.867 2.692-4.58 5.206-5.308.333-.855.826-2.106 1.287-3.029a20.112 20.112 0 0 1-.104-2.171c0-2.659.563-4.992 2.262-6.729.793-.811 1.777-1.433 2.945-1.901C14.502 5.911 18.483 4 23.938 4c5.455 0 9.523 1.911 12.319 4.182 1.167.468 2.151 1.09 2.944 1.901 1.699 1.737 2.263 4.07 2.263 6.729 0 .736-.027 1.465-.105 2.171.461.923.954 2.174 1.288 3.029 2.513.728 4.461 3.441 5.205 5.308.081.205.115.424.115.645v5.318c0 .252-.04.502-.166.72ZM24.325 22.031h-.688a8.52 8.52 0 0 1-.709 1.016c-1.537 1.892-3.833 2.98-7.008 2.98-3.447 0-5.972-.717-7.557-2.514a4.408 4.408 0 0 1-.171-.21l-.195.21v13.155c2.867 1.558 9.02 4.353 15.984 4.353s13.117-2.795 15.984-4.353V23.513l-.195-.21s-.066.091-.171.21c-1.584 1.797-4.11 2.514-7.557 2.514-3.175 0-5.47-1.088-7.008-2.98a8.637 8.637 0 0 1-.709-1.016h-.033.033Zm-1.969-5.864a14.31 14.31 0 0 0 .127-1.785v-.042c-.003-1.537-.339-2.538-.876-3.152-.681-.78-2.09-1.378-5.06-1.057-3.008.326-4.69 1.073-5.643 2.048-.923.944-1.408 2.356-1.408 4.633 0 2.42.348 3.849 1.115 4.719.729.827 2.165 1.499 5.309 1.499 2.417 0 3.799-.786 4.683-1.873.948-1.168 1.482-2.878 1.753-4.99Zm3.25 0c.271 2.112.805 3.822 1.754 4.99.883 1.087 2.265 1.873 4.682 1.873 3.145 0 4.58-.672 5.309-1.499.767-.87 1.116-2.299 1.116-4.719 0-2.277-.485-3.689-1.408-4.633-.954-.975-2.635-1.722-5.644-2.048-2.969-.321-4.378.277-5.06 1.057-.537.614-.873 1.615-.876 3.152v.042c.002.53.042 1.123.127 1.785Z",
            }
            path {
                d: "M28.998 28.516c1.104 0 1.999.895 1.999 1.999v3.998a2 2 0 1 1-3.998 0v-3.998c0-1.104.895-1.999 1.999-1.999Zm-9.996 0c1.104 0 1.999.895 1.999 1.999v3.998a2 2 0 1 1-3.998 0v-3.998c0-1.104.895-1.999 1.999-1.999Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCopilotNotConnected;
impl IconShape for VsCopilotNotConnected {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.357 6.114C14.341 6.104 14.324 6.094 14.308 6.084C13.888 5.835 13.434 5.636 12.957 5.495C12.987 5.306 13.004 5.073 13.004 4.787C13.004 4.021 12.881 3.545 12.634 3.232C12.4 2.936 11.941 2.646 10.921 2.532C9.87695 2.416 9.43295 2.623 9.23995 2.83C9.03595 3.048 8.88095 3.508 8.99795 4.444C9.05595 4.906 9.16195 5.278 9.31395 5.571C8.92695 5.703 8.55695 5.874 8.20995 6.079C7.48695 6.505 6.86095 7.079 6.37395 7.76C5.90195 7.946 5.35895 8.038 4.75095 8.038C4.09895 8.038 3.51395 7.956 3.02395 7.747L3.00095 7.863V12.118C3.26595 12.323 4.28595 12.843 5.57795 13.196C5.69995 13.552 5.85495 13.894 6.03895 14.217C6.18195 14.467 6.34195 14.707 6.51795 14.933C2.65595 14.397 0.00195312 11.923 0.00195312 11.241V9.368C0.0709531 8.744 0.668953 7.659 1.59295 7.274C1.60595 7.204 1.61795 7.131 1.62895 7.056C1.65795 6.873 1.68895 6.672 1.75595 6.444C1.55495 5.936 1.50195 5.361 1.50195 4.789C1.50195 3.918 1.62895 3.02 2.19495 2.304C2.77395 1.571 3.68895 1.18 4.91895 1.043C6.12495 0.909004 7.18095 1.077 7.86295 1.808C7.91195 1.861 7.95895 1.916 8.00195 1.973C8.04495 1.917 8.09495 1.861 8.14495 1.808C8.82695 1.077 9.88295 0.909004 11.089 1.043C12.319 1.18 13.234 1.571 13.813 2.304C14.378 3.02 14.506 3.918 14.506 4.789C14.506 5.241 14.473 5.695 14.36 6.116L14.357 6.114ZM6.76595 2.83C6.57295 2.623 6.12895 2.416 5.08495 2.532C4.06495 2.645 3.60495 2.936 3.37195 3.232C3.12495 3.545 3.00195 4.021 3.00195 4.787C3.00195 5.579 3.13095 5.958 3.30995 6.157C3.47195 6.338 3.82895 6.537 4.75195 6.537C5.60595 6.537 6.09095 6.301 6.38995 5.996C6.70495 5.674 6.91695 5.17 7.00795 4.444C7.12495 3.508 6.96995 3.048 6.76595 2.83Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M11.254 7.75C12.189 7.75 13.068 8.114 13.729 8.775C15.094 10.14 15.094 12.36 13.729 13.725C13.068 14.386 12.189 14.75 11.254 14.75C10.319 14.75 9.43995 14.386 8.77895 13.725C7.41395 12.36 7.41395 10.14 8.77895 8.775C9.43995 8.114 10.319 7.75 11.254 7.75ZM11.254 6.75C10.102 6.75 8.95095 7.189 8.07195 8.068C6.31495 9.825 6.31495 12.675 8.07195 14.432C8.95095 15.311 10.102 15.75 11.254 15.75C12.406 15.75 13.557 15.311 14.436 14.432C16.193 12.675 16.193 9.825 14.436 8.068C13.557 7.189 12.406 6.75 11.254 6.75Z",
            }
            path {
                d: "M12.668 9.086C12.476 9.086 12.284 9.159 12.138 9.306L11.254 10.19L10.37 9.306C10.224 9.16 10.032 9.086 9.83995 9.086C9.64795 9.086 9.45595 9.159 9.30995 9.306C9.01695 9.599 9.01695 10.074 9.30995 10.367L10.194 11.251L9.30995 12.135C9.01695 12.428 9.01695 12.903 9.30995 13.196C9.45595 13.342 9.64795 13.416 9.83995 13.416C10.032 13.416 10.224 13.343 10.37 13.196L11.254 12.312L12.138 13.196C12.284 13.342 12.476 13.416 12.668 13.416C12.86 13.416 13.052 13.343 13.198 13.196C13.491 12.903 13.491 12.428 13.198 12.135L12.314 11.251L13.198 10.367C13.491 10.074 13.491 9.599 13.198 9.306C13.052 9.16 12.86 9.086 12.668 9.086Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCopilotWarningLarge;
impl IconShape for VsCopilotWarningLarge {
    fn view_box(&self) -> &str {
        "0 0 47 43"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M25.5003,37.1833c-0.9549,-1.6764 -1.5003,-3.6161 -1.5003,-5.6833c0,-4.2225 2.2757,-7.9135 5.6675,-9.9133c1.71,-1.0083 3.7037,-1.5867 5.8325,-1.5867c2.137,0 4.138,0.5829 5.8522,1.5984c3.381,2.0025 5.6478,5.6873 5.6478,9.9016c0,6.3513 -5.1487,11.5 -11.5,11.5c-2.6488,0 -5.0884,-0.8957 -7.0327,-2.4004c-1.2003,-0.929 -2.2117,-2.0902 -2.967,-3.4163zM33.5833,30.8612c0,1.0585 0.8582,1.9166 1.9167,1.9166c1.0585,0 1.9167,-0.8581 1.9167,-1.9166v-5.7501c0,-1.0578 -0.8569,-1.9155 -1.9147,-1.9167c-1.0585,0 -1.9187,0.8582 -1.9187,1.9167zM38.0556,37.25c0,-1.4114 -1.1442,-2.5555 -2.5556,-2.5555c-1.4114,0 -2.5556,1.1441 -2.5556,2.5555c0,1.4114 1.1442,2.5556 2.5556,2.5556c1.4114,0 2.5556,-1.1442 2.5556,-2.5556z",
            }
            path {
                clip_rule: "evenodd",
                d: "M42,18.5c-1,-0.5 -2,-1 -4.3667,-1.3442c0.5708,-0.8898 0.8337,-2.2436 0.8337,-4.3438c0,-2.277 -0.485,-3.689 -1.408,-4.633c-0.954,-0.975 -2.635,-1.722 -5.644,-2.048c-2.969,-0.321 -4.378,0.277 -5.06,1.057c-0.537,0.614 -0.873,1.615 -0.876,3.152v0.042c0.002,0.53 0.042,1.123 0.127,1.785c0.271,2.112 0.805,3.822 1.754,4.99c0.4355,0.5362 0.9925,0.9991 1.7352,1.3306c-1.0014,0.4938 -1.9375,1.0998 -2.7919,1.8016c-0.4698,-0.363 -0.8923,-0.7784 -1.2693,-1.2422c-0.2603,-0.3213 -0.4973,-0.6608 -0.709,-1.016h-0.688c-0.2114,0.3554 -0.4483,0.695 -0.709,1.016c-1.537,1.892 -3.833,2.98 -7.008,2.98c-3.447,0 -5.972,-0.717 -7.557,-2.514c-0.05914,-0.0682 -0.11616,-0.1383 -0.171,-0.21l-0.195,0.21v13.155c2.6002,1.413 7.9032,3.8435 14.0627,4.2836c0.448,1.1034 1.028,2.1392 1.7203,3.0878c-12.0233,-0.0773 -21.90852,-7.0649 -23.619,-10.0364c-0.09651,-0.1752 -0.15159,-0.3702 -0.161,-0.57v-5.615c0.012,-0.17 0.047,-0.338 0.11,-0.498c0.744,-1.867 2.692,-4.58 5.206,-5.308c0.333,-0.855 0.826,-2.106 1.287,-3.029c-0.07377,-0.7213 -0.10849,-1.446 -0.104,-2.171c0,-2.659 0.563,-4.992 2.262,-6.729c0.793,-0.811 1.777,-1.433 2.945,-1.901c2.796,-2.271 6.777,-4.182 12.232,-4.182c5.455,0 9.523,1.911 12.319,4.182c1.167,0.468 2.151,1.09 2.944,1.901c1.699,1.737 2.263,4.07 2.263,6.729c0,0.736 -0.027,1.465 -0.105,2.171c0.461,0.923 0.954,2.174 1.288,3.029c0.0444,0.1403 0.0874,0.2966 0.1022,0.3702c0.0149,0.0737 0.0946,0.4342 0.1071,0.6155zM22.483,10.34c-0.003,-1.537 -0.339,-2.538 -0.876,-3.152c-0.681,-0.78 -2.09,-1.378 -5.06,-1.057c-3.008,0.326 -4.69,1.073 -5.643,2.048c-0.923,0.944 -1.408,2.356 -1.408,4.633c0,2.42 0.348,3.849 1.115,4.719c0.729,0.827 2.165,1.499 5.309,1.499c2.417,0 3.799,-0.786 4.683,-1.873c0.948,-1.168 1.482,-2.878 1.753,-4.99c0.0795,-0.5918 0.1219,-1.1879 0.127,-1.785z",
                fill_rule: "evenodd",
            }
            path {
                d: "M21.001,30.513c0.0085,0.2678 -0.037,0.5346 -0.1336,0.7846c-0.0967,0.2499 -0.2425,0.4779 -0.429,0.6703c-0.1865,0.1925 -0.4097,0.3455 -0.6564,0.45c-0.2468,0.1045 -0.512,0.1583 -0.78,0.1583c-0.268,0 -0.5332,-0.0538 -0.78,-0.1583c-0.2467,-0.1045 -0.4699,-0.2575 -0.6564,-0.45c-0.1865,-0.1924 -0.3323,-0.4204 -0.429,-0.6703c-0.0966,-0.25 -0.1421,-0.5168 -0.1336,-0.7846v-3.998c0,-1.104 0.895,-1.999 1.999,-1.999c1.104,0 1.999,0.895 1.999,1.999z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCopilotWarning;
impl IconShape for VsCopilotWarning {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.86079 1.76938C7.91028 1.8224 7.95663 1.87717 8 1.93356C8.04336 1.87717 8.08972 1.8224 8.13921 1.76938C8.82116 1.03872 9.87702 0.870477 11.0828 1.00446C12.3131 1.14115 13.2283 1.53222 13.8072 2.26504C14.3725 2.98056 14.5 3.87915 14.5 4.74987C14.5 5.21326 14.4653 5.67866 14.3451 6.10875C13.9119 5.84737 13.4429 5.63921 12.9473 5.49331C12.9809 5.29773 13 5.05436 13 4.74987C13 3.98422 12.8774 3.50781 12.6303 3.19493C12.3967 2.89934 11.9369 2.60859 10.9172 2.49528C9.87298 2.37926 9.42884 2.58602 9.23579 2.79286C9.03195 3.01126 8.87725 3.47123 8.99421 4.40684C9.05402 4.88539 9.16638 5.26766 9.32639 5.56486C8.14474 5.96456 7.12813 6.72232 6.40569 7.70903C5.92566 7.90339 5.37217 7.99987 4.75 7.99987C4.09776 7.99987 3.51325 7.91826 3.0231 7.70862L3 7.82413V12.0852C3.02086 12.0974 3.04268 12.1101 3.06543 12.1232C3.32878 12.2747 3.71567 12.4793 4.19916 12.6844C4.59656 12.853 5.05487 13.0201 5.56004 13.1588C5.77061 13.7867 6.08221 14.3683 6.47552 14.8842C5.35549 14.7159 4.37511 14.3885 3.61334 14.0653C3.06559 13.8329 2.62435 13.6 2.31739 13.4234C2.16373 13.335 2.0432 13.2604 1.95925 13.2066C1.91725 13.1797 1.88437 13.158 1.86101 13.1423L1.83316 13.1235L1.82736 13.1195L1.81237 13.1094C1.79971 13.1008 1.7818 13.0886 1.75941 13.0731C1.71466 13.0422 1.65169 12.9981 1.57651 12.9438C1.42716 12.836 1.2249 12.6844 1.01986 12.5135C0.819 12.3461 0.595113 12.1435 0.414932 11.9333C0.3249 11.8282 0.230849 11.7042 0.156031 11.567C0.0857453 11.4382 0 11.2394 1.90735e-06 10.9999L0 9.73594C0 8.69432 0.588507 7.74209 1.52017 7.27626L1.5865 7.24309L1.75413 6.40494C1.55295 5.89699 1.5 5.3219 1.5 4.74987C1.5 3.87915 1.62745 2.98056 2.19275 2.26504C2.77172 1.53222 3.68694 1.14115 4.91717 1.00446C6.12298 0.870477 7.17884 1.03872 7.86079 1.76938ZM6.76421 2.79286C6.57116 2.58602 6.12702 2.37926 5.08282 2.49528C4.06305 2.60859 3.60327 2.89934 3.36974 3.19493C3.12255 3.50781 3 3.98422 3 4.74987C3 5.54218 3.12905 5.92064 3.3082 6.12028C3.47045 6.30109 3.82768 6.49987 4.75 6.49987C5.60367 6.49987 6.08903 6.26426 6.38811 5.95894C6.70349 5.63698 6.91507 5.13264 7.00579 4.40684C7.12274 3.47123 6.96805 3.01126 6.76421 2.79286Z",
            }
            path {
                d: "M8.49808 14.8106C9.25887 15.3994 10.2135 15.7499 11.25 15.7499C13.7353 15.7499 15.75 13.7352 15.75 11.2499C15.75 9.60081 14.863 8.15893 13.54 7.37532C12.8692 6.97796 12.0862 6.74987 11.25 6.74987C10.417 6.74987 9.63686 6.97621 8.96772 7.37074C7.64049 8.1533 6.75 9.59759 6.75 11.2499C6.75 12.0588 6.96343 12.8178 7.33706 13.4738C7.63262 13.9927 8.02841 14.4471 8.49808 14.8106ZM10.5 8.74987C10.5 8.33566 10.8366 7.99987 11.2508 7.99987C11.6647 8.00031 12 8.33593 12 8.74987V10.9999C12 11.4141 11.6642 11.7499 11.25 11.7499C10.8358 11.7499 10.5 11.4141 10.5 10.9999V8.74987ZM11.25 14.4999C10.6977 14.4999 10.25 14.0522 10.25 13.4999C10.25 12.9476 10.6977 12.4999 11.25 12.4999C11.8023 12.4999 12.25 12.9476 12.25 13.4999C12.25 14.0522 11.8023 14.4999 11.25 14.4999Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCopilot;
impl IconShape for VsCopilot {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.25 9.016C6.66421 9.016 7 9.35089 7 9.76399V11.26C7 11.6731 6.66421 12.008 6.25 12.008C5.83579 12.008 5.5 11.6731 5.5 11.26V9.76399C5.5 9.35089 5.83579 9.016 6.25 9.016Z",
            }
            path {
                d: "M10.5 9.76399C10.5 9.35089 10.1642 9.016 9.75 9.016C9.33579 9.016 9 9.35089 9 9.76399V11.26C9 11.6731 9.33579 12.008 9.75 12.008C10.1642 12.008 10.5 11.6731 10.5 11.26V9.76399Z",
            }
            path {
                d: "M7.86079 1.80482C7.91028 1.8577 7.95663 1.91232 8 1.96856C8.04337 1.91232 8.08972 1.8577 8.13921 1.80482C8.82116 1.07611 9.87702 0.90832 11.0828 1.04194C12.3131 1.17827 13.2283 1.56829 13.8072 2.29916C14.3725 3.01276 14.5 3.90895 14.5 4.77735C14.5 5.34785 14.447 5.92141 14.2459 6.428L14.4135 7.26391L14.4798 7.29699C15.4115 7.76158 16 8.71126 16 9.7501V11.0107C16 11.2495 15.9143 11.4478 15.844 11.5763C15.7691 11.7131 15.6751 11.8368 15.5851 11.9416C15.4049 12.1512 15.181 12.3534 14.9801 12.5202C14.7751 12.6907 14.5728 12.8419 14.4235 12.9494C14.1842 13.1217 13.9389 13.2807 13.6826 13.4277C13.3756 13.6038 12.9344 13.8361 12.3867 14.0679C11.2956 14.5296 9.75604 15 8 15C6.24396 15 4.70442 14.5296 3.61334 14.0679C3.06559 13.8361 2.62435 13.6038 2.31739 13.4277C2.0611 13.2807 1.81581 13.1217 1.57651 12.9494C1.42716 12.8419 1.2249 12.6907 1.01986 12.5202C0.819 12.3534 0.595113 12.1512 0.414932 11.9416C0.3249 11.8368 0.230849 11.7131 0.156031 11.5763C0.0857453 11.4478 0 11.2495 1.90735e-06 11.0107L0 9.7501C0 8.71126 0.588507 7.76158 1.52017 7.29699L1.5865 7.26391L1.75413 6.42799C1.55295 5.9214 1.5 5.34785 1.5 4.77735C1.5 3.90895 1.62745 3.01276 2.19275 2.29916C2.77172 1.56829 3.68694 1.17827 4.91718 1.04194C6.12298 0.90832 7.17884 1.07611 7.86079 1.80482ZM3.0231 7.7282L3 7.8434V12.0931C3.02086 12.1053 3.04268 12.1179 3.06543 12.131C3.32878 12.2821 3.71567 12.4861 4.19916 12.6907C5.17058 13.1017 6.50604 13.504 8 13.504C9.49396 13.504 10.8294 13.1017 11.8008 12.6907C12.2843 12.4861 12.6712 12.2821 12.9346 12.131C12.9573 12.1179 12.9791 12.1053 13 12.0931V7.8434L12.9769 7.7282C12.4867 7.93728 11.9022 8.01867 11.25 8.01867C10.1037 8.01867 9.19051 7.69201 8.54033 7.03004C8.3213 6.80703 8.14352 6.55741 8 6.28924C7.85648 6.55741 7.6787 6.80703 7.45967 7.03004C6.80949 7.69201 5.89633 8.01867 4.75 8.01867C4.09776 8.01867 3.51325 7.93728 3.0231 7.7282ZM6.76421 2.82557C6.57116 2.61928 6.12702 2.41307 5.08282 2.52878C4.06306 2.64179 3.60328 2.93176 3.36975 3.22656C3.12255 3.53861 3 4.01374 3 4.77735C3 5.56754 3.12905 5.94499 3.3082 6.1441C3.47045 6.32443 3.82768 6.52267 4.75 6.52267C5.60367 6.52267 6.08903 6.28769 6.38811 5.98319C6.70349 5.66209 6.91507 5.1591 7.00579 4.43524C7.12274 3.50212 6.96805 3.04338 6.76421 2.82557ZM9.23579 2.82557C9.03195 3.04338 8.87726 3.50212 8.99421 4.43524C9.08493 5.1591 9.29651 5.66209 9.61189 5.98319C9.91097 6.28769 10.3963 6.52267 11.25 6.52267C12.1723 6.52267 12.5295 6.32443 12.6918 6.1441C12.871 5.94499 13 5.56754 13 4.77735C13 4.01374 12.8775 3.53861 12.6303 3.22656C12.3967 2.93176 11.9369 2.64179 10.9172 2.52878C9.87298 2.41307 9.42884 2.61928 9.23579 2.82557Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCopy;
impl IconShape for VsCopy {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 4l1-1h5.414L14 6.586V14l-1 1H5l-1-1V4zm9 3l-3-3H5v10h8V7z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 1L2 2v10l1 1V2h6.414l-1-1H3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCoverage;
impl IconShape for VsCoverage {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.88415 8.32009L6.38415 11.3201L5.64648 11.3535L4.14648 9.85355L4.85359 9.14644L5.96644 10.2593L8.11593 7.6799L8.88415 8.32009Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M11.9803 9.96948C12.6105 9.33654 13 8.46376 13 7.5C13 5.567 11.433 4 9.5 4C9.03097 4 8.58349 4.09226 8.17472 4.25962C7.69756 4.10726 7.19209 4.01826 6.66821 4.00252C7.44155 3.37561 8.42691 3 9.5 3C11.9853 3 14 5.01472 14 7.5C14 9.2103 13.0459 10.6978 11.6408 11.4591C11.8197 10.99 11.9362 10.4901 11.9803 9.96948Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M6.5 13C4.567 13 3 11.433 3 9.5C3 7.567 4.567 6 6.5 6C8.433 6 10 7.567 10 9.5C10 11.433 8.433 13 6.5 13ZM6.5 14C8.98528 14 11 11.9853 11 9.5C11 7.01472 8.98528 5 6.5 5C4.01472 5 2 7.01472 2 9.5C2 11.9853 4.01472 14 6.5 14Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsCreditCard;
impl IconShape for VsCreditCard {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14 5v1H2V5h12zM2 7h12v5H2V7zm12-3H2a1 1 0 0 0-1 1v7a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V5a1 1 0 0 0-1-1zm-3 6h2v1h-2v-1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDash;
impl IconShape for VsDash {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 8h6v1H5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDashboard;
impl IconShape for VsDashboard {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3.889 2.095a6.5 6.5 0 1 1 7.222 10.81A6.5 6.5 0 0 1 3.89 2.094zm.555 9.978A5.5 5.5 0 0 0 7.5 13 5.506 5.506 0 0 0 13 7.5a5.5 5.5 0 1 0-8.556 4.573zM10.294 4l.706.707-2.15 2.15a1.514 1.514 0 1 1-.707-.707L10.293 4zM7.221 7.916a.5.5 0 1 0 .556-.832.5.5 0 0 0-.556.832zm4.286-2.449l-.763.763c.166.403.253.834.255 1.27a3.463 3.463 0 0 1-.5 1.777l.735.735a4.477 4.477 0 0 0 .274-4.545h-.001zM8.733 4.242A3.373 3.373 0 0 0 7.5 4 3.5 3.5 0 0 0 4 7.5a3.46 3.46 0 0 0 .5 1.777l-.734.735A4.5 4.5 0 0 1 9.5 3.473l-.767.769z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDatabase;
impl IconShape for VsDatabase {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 3.5C13 2.119 10.761 1 8 1S3 2.119 3 3.5c0 .04.02.077.024.117H3v8.872l.056.357C3.336 14.056 5.429 15 8 15c2.571 0 4.664-.944 4.944-2.154l.056-.357V3.617h-.024c.004-.04.024-.077.024-.117zM8 2.032c2.442 0 4 .964 4 1.468s-1.558 1.468-4 1.468S4 4 4 3.5s1.558-1.468 4-1.468zm4 10.458l-.03.131C11.855 13.116 10.431 14 8 14s-3.855-.884-3.97-1.379L4 12.49v-7.5A7.414 7.414 0 0 0 8 6a7.414 7.414 0 0 0 4-1.014v7.504z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugAll;
impl IconShape for VsDebugAll {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.29333 9.00631L6.41333 9.88552C6.27949 9.34717 5.96917 8.86905 5.53181 8.52735C5.09445 8.18564 4.55521 8 4 8C3.44479 8 2.90555 8.18564 2.46819 8.52735C2.03083 8.86905 1.72051 9.34717 1.58667 9.88552L0.706667 9.00631L0 9.71234L1.14667 10.858L1 11.0045V12.0036H0V13.0027H1V13.056C1.051 13.3815 1.14283 13.6993 1.27333 14.0018L0 15.294L0.706667 16L1.80667 14.901C2.06838 15.2346 2.40078 15.5062 2.78001 15.6962C3.15924 15.8862 3.57587 15.99 4 16C4.42413 15.99 4.84076 15.8862 5.21999 15.6962C5.59922 15.5062 5.93162 15.2346 6.19333 14.901L7.29333 16L8 15.294L6.72667 14.0018C6.85879 13.6929 6.95065 13.3683 7 13.036V12.9694H8V12.0036H7V11.0045L6.85333 10.858L8 9.71234L7.29333 9.00631ZM4 9.00631C4.39782 9.00631 4.77936 9.16421 5.06066 9.44526C5.34196 9.72631 5.5 10.1075 5.5 10.505H2.5C2.5 10.1075 2.65804 9.72631 2.93934 9.44526C3.22064 9.16421 3.60218 9.00631 4 9.00631ZM6 13.0027C5.95116 13.5161 5.72476 13.9965 5.35974 14.3612C4.99472 14.7259 4.5139 14.9521 4 15.0009C3.4861 14.9521 3.00528 14.7259 2.64026 14.3612C2.27524 13.9965 2.04884 13.5161 2 13.0027V11.5041H6V13.0027Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M3.77951 2L2.99951 2.41V7H3.99951V3.35L11.5995 8.42L9 10.1507V11.3497L12.7795 8.83V8L3.77951 2ZM9 13.3497V12.1482L14.5995 8.42006L6.99951 3.35006V2.14673L15.7795 8.00006V8.83006L9 13.3497Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugAltSmall;
impl IconShape for VsDebugAltSmall {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.293 9.006l-.88.88A2.484 2.484 0 0 0 4 8a2.488 2.488 0 0 0-2.413 1.886l-.88-.88L0 9.712l1.147 1.146-.147.146v1H0v.999h1v.053c.051.326.143.643.273.946L0 15.294.707 16l1.1-1.099A2.873 2.873 0 0 0 4 16a2.875 2.875 0 0 0 2.193-1.099L7.293 16 8 15.294l-1.273-1.292A3.92 3.92 0 0 0 7 13.036v-.067h1v-.965H7v-1l-.147-.146L8 9.712l-.707-.706zM4 9.006a1.5 1.5 0 0 1 1.5 1.499h-3A1.498 1.498 0 0 1 4 9.006zm2 3.997A2.217 2.217 0 0 1 4 15a2.22 2.22 0 0 1-2-1.998v-1.499h4v1.499z",
            }
            path {
                clip_rule: "evenodd",
                d: "M5 2.41L5.78 2l9 6v.83L9 12.683v-1.2l4.6-3.063L6 3.35V7H5V2.41z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugAlt;
impl IconShape for VsDebugAlt {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.94 13.5l-1.32 1.32a3.73 3.73 0 0 0-7.24 0L1.06 13.5 0 14.56l1.72 1.72-.22.22V18H0v1.5h1.5v.08c.077.489.214.966.41 1.42L0 22.94 1.06 24l1.65-1.65A4.308 4.308 0 0 0 6 24a4.31 4.31 0 0 0 3.29-1.65L10.94 24 12 22.94 10.09 21c.198-.464.336-.951.41-1.45v-.1H12V18h-1.5v-1.5l-.22-.22L12 14.56l-1.06-1.06zM6 13.5a2.25 2.25 0 0 1 2.25 2.25h-4.5A2.25 2.25 0 0 1 6 13.5zm3 6a3.33 3.33 0 0 1-3 3 3.33 3.33 0 0 1-3-3v-2.25h6v2.25zm14.76-9.9v1.26L13.5 17.37V15.6l8.5-5.37L9 2v9.46a5.07 5.07 0 0 0-1.5-.72V.63L8.64 0l15.12 9.6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugBreakpointConditionalUnverified;
impl IconShape for VsDebugBreakpointConditionalUnverified {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.778 4.674a4 4 0 1 1 4.444 6.652 4 4 0 0 1-4.444-6.652zm.694 5.612a2.75 2.75 0 1 0 3.056-4.572 2.75 2.75 0 0 0-3.056 4.572zM9.5 6.5h-3v1h3v-1zm0 2h-3v1h3v-1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugBreakpointConditional;
impl IconShape for VsDebugBreakpointConditional {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8 4a4 4 0 1 0 0 8 4 4 0 0 0 0-8zm2 5v1H6V9h4zm0-3v1H6V6h4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugBreakpointDataUnverified;
impl IconShape for VsDebugBreakpointDataUnverified {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.931 4h-4.62l-2.31 4 2.31 4h4.62l2.31-4-2.31-4zm-.75 6.7h-3.12L4.501 8l1.56-2.7h3.12l1.56 2.7-1.56 2.7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugBreakpointData;
impl IconShape for VsDebugBreakpointData {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12.238 8l-2.31 4H5.31L3 8l2.31-4h4.618l2.31 4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugBreakpointFunctionUnverified;
impl IconShape for VsDebugBreakpointFunctionUnverified {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 11h8L8 4l-4 7zm2.154-1.25h3.692L8 6.52 6.154 9.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugBreakpointFunction;
impl IconShape for VsDebugBreakpointFunction {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 4l4 6.905H4L8 4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugBreakpointLogUnverified;
impl IconShape for VsDebugBreakpointLogUnverified {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3.02 7.98L8 3l4.98 4.98L8 12.96 3.02 7.98zM8 10.77l2.79-2.79L8 5.19 5.21 7.98 8 10.77z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugBreakpointLog;
impl IconShape for VsDebugBreakpointLog {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 3l5 5-5 5-5-5 5-5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugBreakpointUnsupported;
impl IconShape for VsDebugBreakpointUnsupported {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.326 10.222a4 4 0 1 0-6.653-4.444 4 4 0 0 0 6.653 4.444zM8.65 10H7.4v1h1.25v-1zM7.4 9V5h1.25v4H7.4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugConsole;
impl IconShape for VsDebugConsole {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.04 1.361l.139-.057H21.32l.14.057 1.178 1.179.057.139V16.82l-.057.14-1.179 1.178-.139.057H14V18a1.99 1.99 0 0 0-.548-1.375h7.673V2.875H7.375v7.282a5.73 5.73 0 0 0-1.571-.164V2.679l.057-.14L7.04 1.362zm9.531 9.452l-2.809 2.8a2 2 0 0 0-.348-.467l-.419-.42 2.236-2.235-3.606-3.694.813-.833 4.133 4.133v.716zM9.62 14.82l1.32-1.32L12 14.56l-1.72 1.72.22.22V18H12v1.45h-1.5v.1a5.888 5.888 0 0 1-.41 1.45L12 22.94 10.94 24l-1.65-1.65A4.308 4.308 0 0 1 6 24a4.31 4.31 0 0 1-3.29-1.65L1.06 24 0 22.94 1.91 21a5.889 5.889 0 0 1-.41-1.42v-.08H0V18h1.5v-1.5l.22-.22L0 14.56l1.06-1.06 1.32 1.32a3.73 3.73 0 0 1 7.24 0zm-2.029-.661A2.25 2.25 0 0 0 3.75 15.75h4.5a2.25 2.25 0 0 0-.659-1.591zm.449 7.38A3.33 3.33 0 0 0 9 19.5v-2.25H3v2.25a3.33 3.33 0 0 0 3 3 3.33 3.33 0 0 0 2.04-.96z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugContinueSmall;
impl IconShape for VsDebugContinueSmall {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 2H3V14H4V2ZM7.29062 2.59314L6.5 3.00001V13L7.29062 13.4069L14.2906 8.40687V7.59314L7.29062 2.59314ZM13.1398 8.00001L7.5 12.0284V3.9716L13.1398 8.00001Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugContinue;
impl IconShape for VsDebugContinue {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.5 2H4v12H2.5V2zm4.936.39L6.25 3v10l1.186.61 7-5V7.39l-7-5zM12.71 8l-4.96 3.543V4.457L12.71 8z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugCoverage;
impl IconShape for VsDebugCoverage {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 2.41L5.78 2L14.78 8V8.83L9 12.6833V11.4826L13.6 8.42L6 3.35V7H5V2.41Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M6.13041 12.1236C5.94796 12.3032 5.80777 12.503 5.70927 12.7232C5.61337 12.9427 5.56516 13.181 5.56516 13.4389V14.3007C5.56516 14.3973 5.54694 14.4886 5.51016 14.5741C5.4738 14.6587 5.42387 14.7328 5.36036 14.7961C5.29687 14.8594 5.2225 14.9091 5.13774 14.9453C5.05203 14.9819 4.96049 15 4.86366 15H3.9988C3.90197 15 3.81043 14.9819 3.72472 14.9453C3.63996 14.9091 3.5656 14.8594 3.5021 14.7961C3.4386 14.7328 3.38866 14.6587 3.3523 14.5741C3.31552 14.4886 3.2973 14.3973 3.2973 14.3007V13.4389C3.2973 13.1811 3.248 12.9428 3.1499 12.7234C3.05368 12.5033 2.91448 12.3031 2.73205 12.1236C2.49791 11.8926 2.31713 11.6346 2.19041 11.35C2.0633 11.0644 2 10.7552 2 10.4228C2 10.2005 2.02876 9.98586 2.08641 9.77906C2.14392 9.57279 2.22565 9.38 2.33166 9.20087C2.43754 9.01972 2.56419 8.85567 2.71156 8.70884C2.85886 8.56206 3.02231 8.4359 3.20182 8.33042C3.38373 8.22488 3.57848 8.14334 3.78542 8.08606C3.99288 8.02865 4.2082 8 4.43123 8C4.65426 8 4.86959 8.02865 5.07704 8.08606C5.28398 8.14334 5.4774 8.22475 5.65714 8.33035C5.8389 8.43582 6.00353 8.56199 6.15091 8.70884C6.29827 8.85567 6.42492 9.01972 6.53079 9.20086C6.63681 9.37999 6.71854 9.57278 6.77605 9.77906C6.8337 9.98586 6.86246 10.2005 6.86246 10.4228C6.86246 10.7552 6.79916 11.0644 6.67206 11.35C6.54533 11.6346 6.36456 11.8926 6.13041 12.1236ZM5.02703 13.1154H3.83544V14.3007C3.83544 14.3443 3.8508 14.3814 3.88401 14.4145C3.91724 14.4476 3.95465 14.4631 3.9988 14.4631H4.86366C4.90781 14.4631 4.94523 14.4476 4.97845 14.4145C5.01166 14.3814 5.02703 14.3443 5.02703 14.3007V13.1154Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugDisconnect;
impl IconShape for VsDebugDisconnect {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13.617 3.844a2.87 2.87 0 0 0-.451-.868l1.354-1.36L13.904 1l-1.36 1.354a2.877 2.877 0 0 0-.868-.452 3.073 3.073 0 0 0-2.14.075 3.03 3.03 0 0 0-.991.664L7 4.192l4.327 4.328 1.552-1.545c.287-.287.508-.618.663-.992a3.074 3.074 0 0 0 .075-2.14zm-.889 1.804a2.15 2.15 0 0 1-.471.705l-.93.93-3.09-3.09.93-.93a2.15 2.15 0 0 1 .704-.472 2.134 2.134 0 0 1 1.689.007c.264.114.494.271.69.472.2.195.358.426.472.69a2.134 2.134 0 0 1 .007 1.688zm-4.824 4.994l1.484-1.545-.616-.622-1.49 1.551-1.86-1.859 1.491-1.552L6.291 6 4.808 7.545l-.616-.615-1.551 1.545a3 3 0 0 0-.663.998 3.023 3.023 0 0 0-.233 1.169c0 .332.05.656.15.97.105.31.258.597.459.862L1 13.834l.615.615 1.36-1.353c.265.2.552.353.862.458.314.1.638.15.97.15.406 0 .796-.077 1.17-.232.378-.155.71-.376.998-.663l1.545-1.552-.616-.615zm-2.262 2.023a2.16 2.16 0 0 1-.834.164c-.301 0-.586-.057-.855-.17a2.278 2.278 0 0 1-.697-.466 2.28 2.28 0 0 1-.465-.697 2.167 2.167 0 0 1-.17-.854 2.16 2.16 0 0 1 .642-1.545l.93-.93 3.09 3.09-.93.93a2.22 2.22 0 0 1-.711.478z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugLineByLine;
impl IconShape for VsDebugLineByLine {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 10V9h9v1H6zm4-4h5v1h-5V6zm5-3v1H6V3h9zm-9 9v1h9v-1H6z",
            }
            path {
                clip_rule: "evenodd",
                d: "M1 2.795l.783-.419 5.371 3.581v.838l-5.371 3.581L1 9.957V2.795zm1.007.94v5.281l3.96-2.64-3.96-2.64z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugPause;
impl IconShape for VsDebugPause {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.5 3H6v10H4.5V3zm7 0v10H10V3h1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugRerun;
impl IconShape for VsDebugRerun {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.167 12a3 3 0 0 1-5.74 1.223l-.928.376A4.001 4.001 0 1 0 1 9.556V8.333H0V11l.5.5h2.333v-1H1.568A3 3 0 0 1 7.167 12z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M5 2.41L5.78 2l9 6v.83L10 12.017v-1.2l3.6-2.397L6 3.35V7H5V2.41z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugRestartFrame;
impl IconShape for VsDebugRestartFrame {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1 10V9h5.207a5.48 5.48 0 0 0-.185 1H1zm6.257-3a5.54 5.54 0 0 1 1.08-1H1v1h6.257zM6.6 13a5.465 5.465 0 0 1-.393-1H1v1h5.6zM15 3v1H1V3h14zm-3.36 10.031a2.531 2.531 0 1 0-2.192-3.797h1.068v.844h-1.97l-.421-.422v-2.25h.844v1.032a3.375 3.375 0 1 1-.423 3.412l.782-.318a2.532 2.532 0 0 0 2.313 1.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugRestart;
impl IconShape for VsDebugRestart {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.75 8a4.5 4.5 0 0 1-8.61 1.834l-1.391.565A6.001 6.001 0 0 0 14.25 8 6 6 0 0 0 3.5 4.334V2.5H2v4l.75.75h3.5v-1.5H4.352A4.5 4.5 0 0 1 12.75 8z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugReverseContinue;
impl IconShape for VsDebugReverseContinue {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13.5 2H12v12h1.5V2zm-4.936.39L9.75 3v10l-1.186.61-7-5V7.39l7-5zM3.29 8l4.96 3.543V4.457L3.29 8z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugStackframeActive;
impl IconShape for VsDebugStackframeActive {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 8a2 2 0 1 1-4 0 2 2 0 0 1 4 0z",
            }
            path {
                d: "M14.5 7.15l-4.26-4.74L9.31 2H4.25L3 3.25v9.48l1.25 1.25h5.06l.93-.42 4.26-4.74V7.15zm-5.19 5.58H4.25V3.25h5.06l4.26 4.73-4.26 4.75z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugStackframe;
impl IconShape for VsDebugStackframe {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.5 7.15l-4.26-4.74L9.31 2H4.25L3 3.25v9.48l1.25 1.25h5.06l.93-.42 4.26-4.74V7.15zm-5.19 5.58H4.25V3.25h5.06l4.26 4.73-4.26 4.75z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugStart;
impl IconShape for VsDebugStart {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4.25 3l1.166-.624 8 5.333v1.248l-8 5.334-1.166-.624V3zm1.5 1.401v7.864l5.898-3.932L5.75 4.401z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugStepBack;
impl IconShape for VsDebugStepBack {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.75 5.75v-4h1.5v2.542c1.145-1.359 2.911-2.209 4.84-2.209 3.177 0 5.92 2.307 6.16 5.398l.02.269h-1.5l-.022-.226c-.212-2.195-2.202-3.94-4.656-3.94-1.736 0-3.244.875-4.05 2.166h2.83v1.5H2.707l-.961-.975V5.75h.003zM8 14a2 2 0 1 1 0-4 2 2 0 0 1 0 4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugStepInto;
impl IconShape for VsDebugStepInto {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8 9.532h.542l3.905-3.905-1.061-1.06-2.637 2.61V1H7.251v6.177l-2.637-2.61-1.061 1.06 3.905 3.905H8zm1.956 3.481a2 2 0 1 1-4 0 2 2 0 0 1 4 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugStepOut;
impl IconShape for VsDebugStepOut {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8 1h-.542L3.553 4.905l1.061 1.06 2.637-2.61v6.177h1.498V3.355l2.637 2.61 1.061-1.06L8.542 1H8zm1.956 12.013a2 2 0 1 1-4 0 2 2 0 0 1 4 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugStepOver;
impl IconShape for VsDebugStepOver {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.25 5.75v-4h-1.5v2.542c-1.145-1.359-2.911-2.209-4.84-2.209-3.177 0-5.92 2.307-6.16 5.398l-.02.269h1.501l.022-.226c.212-2.195 2.202-3.94 4.656-3.94 1.736 0 3.244.875 4.05 2.166h-2.83v1.5h4.163l.962-.975V5.75h-.004zM8 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebugStop;
impl IconShape for VsDebugStop {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13 1.99976L14 2.99976V12.9998L13 13.9998H3L2 12.9998L2 2.99976L3 1.99976H13ZM12.7461 3.25057L3.25469 3.25057L3.25469 12.7504H12.7461V3.25057Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDebug;
impl IconShape for VsDebug {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.463 12.86l-.005-.07.005.07zm7.264.69l-3.034-3.049 1.014-1.014 3.209 3.225 3.163-3.163 1.014 1.014-3.034 3.034 3.034 3.05-1.014 1.014-3.209-3.225L8.707 17.6l-1.014-1.014 3.034-3.034z",
            }
            path {
                clip_rule: "evenodd",
                d: "M16.933 5.003V6h1.345l2.843-2.842 1.014 1.014-2.692 2.691.033.085a13.75 13.75 0 0 1 .885 4.912c0 .335-.011.667-.034.995l-.005.075h3.54v1.434h-3.72l-.01.058c-.303 1.653-.891 3.16-1.692 4.429l-.06.094 3.423 3.44-1.017 1.012-3.274-3.29-.099.11c-1.479 1.654-3.395 2.646-5.483 2.646-2.12 0-4.063-1.023-5.552-2.723l-.098-.113-3.209 3.208-1.014-1.014 3.366-3.365-.059-.095c-.772-1.25-1.34-2.725-1.636-4.34l-.01-.057H0V12.93h3.538l-.005-.075a14.23 14.23 0 0 1-.034-.995c0-1.743.31-3.39.863-4.854l.032-.084-2.762-2.776L2.65 3.135 5.5 6h1.427v-.997a5.003 5.003 0 0 1 10.006 0zm-8.572 0V6H15.5v-.997a3.569 3.569 0 0 0-7.138 0zm9.8 2.522l-.034-.09H5.733l-.034.09a12.328 12.328 0 0 0-.766 4.335c0 2.76.862 5.201 2.184 6.92 1.32 1.716 3.036 2.649 4.813 2.649 1.777 0 3.492-.933 4.813-2.65 1.322-1.718 2.184-4.16 2.184-6.919 0-1.574-.28-3.044-.766-4.335z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDesktopDownload;
impl IconShape for VsDesktopDownload {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 15v-1c2 0 2-.6 2-1H1.5l-.5-.5v-10l.5-.5h13l.5.5v9.24l-1-1V3H2v9h5.73l-.5.5 2.5 2.5H4zm7.86 0l2.5-2.5-.71-.7L12 13.45V7h-1v6.44l-1.64-1.65-.71.71 2.5 2.5h.71z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDeviceCameraVideo;
impl IconShape for VsDeviceCameraVideo {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.25 4.74L11 6.62V4.5l-.5-.5h-9l-.5.5v7l.5.5h9l.5-.5v-2l3.25 1.87.75-.47V5.18l-.75-.44zM10 11H2V5h8v6zm4-1l-3-1.7v-.52L14 6v4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDeviceCamera;
impl IconShape for VsDeviceCamera {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.707 3H14.5l.5.5v9l-.5.5h-13l-.5-.5v-9l.5-.5h3.793l.853-.854L6.5 2h3l.354.146.853.854zM2 12h12V4h-3.5l-.354-.146L9.293 3H6.707l-.853.854L5.5 4H2v8zm1.5-7a.5.5 0 1 0 0 1 .5.5 0 0 0 0-1zM8 6a2 2 0 1 1 0 4 2 2 0 0 1 0-4zm0-1a3 3 0 1 0 0 6 3 3 0 0 0 0-6z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDeviceMobile;
impl IconShape for VsDeviceMobile {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4.5 1h8l.5.5v13l-.5.5h-8l-.5-.5v-13l.5-.5zM5 14h7V2H5v12zm2.5-2h2v1h-2v-1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDiffAdded;
impl IconShape for VsDiffAdded {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 1h12l.5.5v12l-.5.5h-12l-.5-.5v-12l.5-.5zM2 13h11V2H2v11z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M8 4H7v3H4v1h3v3h1V8h3V7H8V4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDiffIgnored;
impl IconShape for VsDiffIgnored {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 1h13l.5.5v13l-.5.5h-13l-.5-.5v-13l.5-.5zM2 14h12V2H2v12zm8-10h2v2l-6 6H4v-2l6-6z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDiffModified;
impl IconShape for VsDiffModified {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 1h13l.5.5v13l-.5.5h-13l-.5-.5v-13l.5-.5zM2 2v12h12V2H2zm6 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDiffMultiple;
impl IconShape for VsDiffMultiple {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.70994 1.29L12.7099 4.29L12.9999 5V14L11.9999 15H2.99994L1.99994 14V2L2.99994 1H8.99994L9.70994 1.29ZM2.99994 14H11.9999V5L8.99994 2H2.99994V14ZM7 6H5V7H7V9H8V7H10V6H8V4H7V6ZM5 11H10V12H5V11Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M12.4199 0.999756L14.7099 3.28976L14.9999 3.99976L15 13.9998L14 14.9998L13.9999 3.99976L10.9999 0.999756H12.4199Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDiffRemoved;
impl IconShape for VsDiffRemoved {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 7v1H5V7h5z",
            }
            path {
                clip_rule: "evenodd",
                d: "M1.5 1h12l.5.5v12l-.5.5h-12l-.5-.5v-12l.5-.5zM2 13h11V2H2v11z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDiffRenamed;
impl IconShape for VsDiffRenamed {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 1h13l.5.5v13l-.5.5h-13l-.5-.5v-13l.5-.5zM2 14h12V2H2v12zm2-5h3v3l5-4-5-4v3H4v2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDiffSingle;
impl IconShape for VsDiffSingle {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.7099 1.28902L13.7099 4.28902L13.9999 4.99902V13.999L12.9999 14.999H3.99994L2.99994 13.999V1.99902L3.99994 0.999023H9.99994L10.7099 1.28902ZM3.99994 13.999H12.9999V4.99902L9.99994 1.99902H3.99994V13.999ZM8 5.99902H6V6.99902H8V8.99902H9V6.99902H11V5.99902H9V3.99902H8V5.99902ZM6 10.999H11V11.999H6V10.999Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDiff;
impl IconShape for VsDiff {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 3.5l.5-.5h5l.5.5v9l-.5.5h-5l-.5-.5v-9zM3 12h4V6H3v6zm0-7h4V4H3v1zm6.5-2h5l.5.5v9l-.5.5h-5l-.5-.5v-9l.5-.5zm.5 9h4v-2h-4v2zm0-4h4V4h-4v4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsDiscard;
impl IconShape for VsDiscard {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3.5 2v3.5L4 6h3.5V5H4.979l.941-.941a3.552 3.552 0 1 1 5.023 5.023L5.746 14.28l.72.72 5.198-5.198A4.57 4.57 0 0 0 5.2 3.339l-.7.7V2h-1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsEditSession;
impl IconShape for VsEditSession {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 14V6.5H13V14L12 15H3L2 14V2L3 1H8V2H3V14H12Z",
            }
            path {
                d: "M7.00006 4H8V6H10.0001V7H8.00006V9H7.00006V7H5.00006V6H7.00006V4Z",
            }
            path {
                d: "M10.0001 11H5.00006V12H10.0001V11Z",
            }
            path {
                d: "M13.8867 0.596993V2.28996H12.1938V1.72564H12.7493C12.5964 1.57868 12.4245 1.46405 12.2334 1.38175C12.0424 1.29946 11.8411 1.25831 11.6294 1.25831C11.4737 1.25831 11.3252 1.28035 11.1841 1.32444C11.0431 1.36853 10.9079 1.43025 10.7785 1.50961C10.6492 1.58896 10.5346 1.68449 10.4347 1.79618C10.3347 1.90787 10.2524 2.03278 10.1878 2.17092L9.5 1.84467C9.59993 1.63599 9.72338 1.45082 9.87034 1.28917C10.0173 1.12751 10.1848 0.986434 10.3729 0.865927C10.561 0.745421 10.7609 0.654307 10.9725 0.592584C11.1841 0.530861 11.4031 0.5 11.6294 0.5C11.9498 0.5 12.2569 0.563192 12.5509 0.689577C12.8448 0.815961 13.102 0.993781 13.3224 1.22304V0.596993H13.8867Z",
            }
            path {
                d: "M9.5 5.13714V3.44418H11.193V4.0085H10.6375C10.7903 4.15546 10.9622 4.27008 11.1533 4.35238C11.3443 4.43468 11.5457 4.47583 11.7573 4.47583C11.9131 4.47583 12.0615 4.45378 12.2026 4.40969C12.3437 4.36561 12.4789 4.30388 12.6082 4.22453C12.7375 4.14517 12.8521 4.04965 12.9521 3.93796C13.052 3.82627 13.1343 3.70135 13.199 3.56321L13.8867 3.88946C13.7868 4.09814 13.6633 4.28331 13.5164 4.44496C13.3694 4.60662 13.2019 4.7477 13.0138 4.86821C12.8257 4.98871 12.6258 5.07983 12.4142 5.14155C12.2026 5.20327 11.9836 5.23413 11.7573 5.23413C11.4369 5.23413 11.1298 5.17094 10.8359 5.04456C10.5419 4.91817 10.2848 4.74035 10.0643 4.5111V5.13714H9.5Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsEdit;
impl IconShape for VsEdit {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.23 1h-1.46L3.52 9.25l-.16.22L1 13.59 2.41 15l4.12-2.36.22-.16L15 4.23V2.77L13.23 1zM2.41 13.59l1.51-3 1.45 1.45-2.96 1.55zm3.83-2.06L4.47 9.76l8-8 1.77 1.77-8 8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsEditorLayout;
impl IconShape for VsEditorLayout {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 6.5l-.47-.5H7V1.47L6.53 1H1.47L1 1.47v8.06l.47.47H4v4.53l.47.47h10.06l.47-.47V6.5zM2 9V3h4v6H2zm12 5H5v-4h1.53L7 9.53V8.013h7V14z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsEllipsis;
impl IconShape for VsEllipsis {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 8a1 1 0 1 1-2 0 1 1 0 0 1 2 0zm5 0a1 1 0 1 1-2 0 1 1 0 0 1 2 0zm5 0a1 1 0 1 1-2 0 1 1 0 0 1 2 0z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsEmptyWindow;
impl IconShape for VsEmptyWindow {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 4h3v1H4v3H3V5H0V4h3V1h1v3zM1 14.5V9h1v5h12V7H8V6h6V4H8V3h6.5l.5.5v11l-.5.5h-13l-.5-.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsErrorSmall;
impl IconShape for VsErrorSmall {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.17699 10.1051L8.00026 8.92835L6.82348 10.1051L5.89526 9.17693L7.07205 8.00014L5.89526 6.82335L6.82348 5.89514L8.00026 7.07193L9.17699 5.8952L10.1052 6.82342L8.92848 8.00014L10.1052 9.17687L9.17699 10.1051Z",
            }
            path {
                d: "M12.0002 8C12.0002 10.2091 10.2094 12 8.00024 12C5.79111 12 4.00024 10.2091 4.00024 8C4.00024 5.79086 5.79111 4 8.00024 4C10.2094 4 12.0002 5.79086 12.0002 8ZM11.0002 8C11.0002 6.34315 9.6571 5 8.00024 5C6.34339 5 5.00024 6.34315 5.00024 8C5.00024 9.65685 6.34339 11 8.00024 11C9.6571 11 11.0002 9.65685 11.0002 8Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsError;
impl IconShape for VsError {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.6 1c1.6.1 3.1.9 4.2 2 1.3 1.4 2 3.1 2 5.1 0 1.6-.6 3.1-1.6 4.4-1 1.2-2.4 2.1-4 2.4-1.6.3-3.2.1-4.6-.7-1.4-.8-2.5-2-3.1-3.5C.9 9.2.8 7.5 1.3 6c.5-1.6 1.4-2.9 2.8-3.8C5.4 1.3 7 .9 8.6 1zm.5 12.9c1.3-.3 2.5-1 3.4-2.1.8-1.1 1.3-2.4 1.2-3.8 0-1.6-.6-3.2-1.7-4.3-1-1-2.2-1.6-3.6-1.7-1.3-.1-2.7.2-3.8 1-1.1.8-1.9 1.9-2.3 3.3-.4 1.3-.4 2.7.2 4 .6 1.3 1.5 2.3 2.7 3 1.2.7 2.6.9 3.9.6zM7.9 7.5L10.3 5l.7.7-2.4 2.5 2.4 2.5-.7.7-2.4-2.5-2.4 2.5-.7-.7 2.4-2.5-2.4-2.5.7-.7 2.4 2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsExclude;
impl IconShape for VsExclude {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.412 1H6.588l-.484 2.423-2.056-1.37-1.996 1.995 1.37 2.056L1 6.588v2.824l2.423.484-1.37 2.056 1.995 1.996 2.056-1.37L6.588 15h2.083a4.526 4.526 0 0 1-.917-1.005h-.342l-.288-1.441a4.473 4.473 0 0 1-.067-.334l-.116-.583-.764-.316-2 1.334-.832-.831L4.68 9.823l-.316-.764-2.358-.471V7.412l2.358-.471.316-.764-1.334-2 .831-.832 2 1.335.764-.316.471-2.358h1.176l.471 2.358.764.316 2-1.334.832.831-1.334 2.001.316.764.582.116c.113.018.225.04.335.067l1.441.288v.342c.38.254.719.563 1.005.917V6.588l-2.422-.484 1.37-2.056-1.996-1.996-2.056 1.37L9.412 1zM8 6a2 2 0 0 1 1.875 1.302 4.46 4.46 0 0 0-.9.473 1 1 0 1 0-1.2 1.2 4.46 4.46 0 0 0-.473.9A2 2 0 0 1 8 6zm1.28 2.795a3.5 3.5 0 1 1 4.44 5.41 3.5 3.5 0 0 1-4.44-5.41zM9 11v1h5v-1H9z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsExpandAll;
impl IconShape for VsExpandAll {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 9H4v1h5V9z",
            }
            path {
                d: "M7 12V7H6v5h1z",
            }
            path {
                clip_rule: "evenodd",
                d: "M5 3l1-1h7l1 1v7l-1 1h-2v2l-1 1H3l-1-1V6l1-1h2V3zm1 2h4l1 1v4h2V3H6v2zm4 1H3v7h7V6z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsExport;
impl IconShape for VsExport {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13.086 7l-2.39-2.398.702-.704L15 7.5l-3.602 3.602-.703-.704 2.383-2.382V8H3V7h10.086zM1 4h1v7H1V4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsExtensions;
impl IconShape for VsExtensions {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13.5 1.5L15 0h7.5L24 1.5V9l-1.5 1.5H15L13.5 9V1.5zm1.5 0V9h7.5V1.5H15zM0 15V6l1.5-1.5H9L10.5 6v7.5H18l1.5 1.5v7.5L18 24H1.5L0 22.5V15zm9-1.5V6H1.5v7.5H9zM9 15H1.5v7.5H9V15zm1.5 7.5H18V15h-7.5v7.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsEyeClosed;
impl IconShape for VsEyeClosed {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.47978 1.4797C1.30227 1.65721 1.28614 1.93498 1.43137 2.13072L1.47978 2.1868L4.1695 4.87652C2.88817 5.77616 1.93052 7.11985 1.53259 8.70952C1.46554 8.97738 1.62834 9.24892 1.89621 9.31598C2.16409 9.38298 2.4356 9.22025 2.50266 8.95232C2.85564 7.54225 3.72742 6.35956 4.88944 5.59626L6.09586 6.80278C5.62419 7.28378 5.33334 7.94278 5.33334 8.66965C5.33334 10.1424 6.52724 11.3363 8 11.3363C8.72694 11.3363 9.38587 11.0454 9.86694 10.5738L13.8131 14.5201C14.0084 14.7154 14.3249 14.7154 14.5202 14.5201C14.6977 14.3426 14.7139 14.0649 14.5686 13.8691L14.5202 13.813L10.4445 9.73692L10.4453 9.73592L9.64527 8.93732L7.732 7.02445L7.73334 7.02392L5.81252 5.10513L5.81334 5.10392L5.05782 4.35024L2.18689 1.4797C1.99163 1.28444 1.67504 1.28444 1.47978 1.4797ZM6.80274 7.51025L9.15947 9.86698C8.85947 10.1575 8.4506 10.3363 8 10.3363C7.07954 10.3363 6.33334 9.59012 6.33334 8.66965C6.33334 8.21905 6.51216 7.81018 6.80274 7.51025ZM8 3.66658C7.33314 3.66658 6.68607 3.7653 6.07406 3.94992L6.89874 4.77404C7.25594 4.70346 7.62427 4.66658 8 4.66658C10.6154 4.66658 12.8733 6.45342 13.4981 8.95538C13.565 9.22325 13.8364 9.38618 14.1043 9.31932C14.3723 9.25238 14.5352 8.98098 14.4683 8.71305C13.7329 5.7684 11.077 3.66658 8 3.66658ZM8.1298 6.0061L10.664 8.53992C10.5961 7.16865 9.49814 6.07168 8.1298 6.0061Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsEye;
impl IconShape for VsEye {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.99993 6.00316C9.47266 6.00316 10.6666 7.19708 10.6666 8.66981C10.6666 10.1426 9.47266 11.3365 7.99993 11.3365C6.52715 11.3365 5.33324 10.1426 5.33324 8.66981C5.33324 7.19708 6.52715 6.00316 7.99993 6.00316ZM7.99993 7.00315C7.07946 7.00315 6.33324 7.74935 6.33324 8.66981C6.33324 9.59028 7.07946 10.3365 7.99993 10.3365C8.9204 10.3365 9.6666 9.59028 9.6666 8.66981C9.6666 7.74935 8.9204 7.00315 7.99993 7.00315ZM7.99993 3.66675C11.0756 3.66675 13.7307 5.76675 14.4673 8.70968C14.5344 8.97755 14.3716 9.24908 14.1037 9.31615C13.8358 9.38315 13.5643 9.22041 13.4973 8.95248C12.8713 6.45205 10.6141 4.66675 7.99993 4.66675C5.38454 4.66675 3.12664 6.45359 2.50182 8.95555C2.43491 9.22341 2.16348 9.38635 1.89557 9.31948C1.62766 9.25255 1.46471 8.98115 1.53162 8.71321C2.26701 5.76856 4.9229 3.66675 7.99993 3.66675Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFeedback;
impl IconShape for VsFeedback {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.5 1L4 1.5V3.02746C4.16417 3.00932 4.331 3 4.5 3C4.669 3 4.83583 3.00932 5 3.02746V2H14V7H12.2929L11 8.29289V7H8.97254C8.99068 7.16417 9 7.331 9 7.5C9 7.669 8.99068 7.83583 8.97254 8H10V9.5L10.8536 9.85355L12.7071 8H14.5L15 7.5V1.5L14.5 1H4.5Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M6.41705 10.4288C7.37039 9.80348 8 8.72527 8 7.5C8 5.567 6.433 4 4.5 4C2.567 4 1 5.567 1 7.5C1 8.72527 1.62961 9.80348 2.58295 10.4288C2.11364 10.6498 1.68557 10.9505 1.31802 11.318C0.900156 11.7359 0.568688 12.232 0.342542 12.7779C0.180451 13.1692 0.0747425 13.5807 0.0278638 14C0.00933826 14.1657 0 14.3326 0 14.5V15H1L0.999398 14.5C0.999398 14.4784 0.999599 14.4567 1 14.4351C1.00811 13.9975 1.09823 13.5651 1.26587 13.1604C1.44179 12.7357 1.69964 12.3498 2.0247 12.0247C2.34976 11.6996 2.73566 11.4418 3.16038 11.2659C3.57088 11.0958 4.00986 11.0056 4.45387 10.9997C4.46922 10.9999 4.4846 11 4.5 11C4.5154 11 4.53078 10.9999 4.54613 10.9997C4.99014 11.0056 5.42912 11.0958 5.83962 11.2659C6.26433 11.4418 6.65024 11.6996 6.9753 12.0247C7.30036 12.3498 7.55821 12.7357 7.73413 13.1604C7.90177 13.5651 7.99189 13.9975 8 14.4351C8.0004 14.4567 8.0006 14.4784 8.0006 14.5L8 15H9V14.5C9 14.3326 8.99066 14.1657 8.97214 14C8.92526 13.5807 8.81955 13.1692 8.65746 12.7779C8.43131 12.232 8.09984 11.7359 7.68198 11.318C7.31443 10.9505 6.88636 10.6498 6.41705 10.4288ZM4.5 10C3.11929 10 2 8.88071 2 7.5C2 6.11929 3.11929 5 4.5 5C5.88071 5 7 6.11929 7 7.5C7 8.88071 5.88071 10 4.5 10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFileBinary;
impl IconShape for VsFileBinary {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.57 1.14l3.28 3.3.15.36v9.7l-.5.5h-11l-.5-.5v-13l.5-.5h7.72l.35.14zM3 2v12h10V5l-3-3H3zm1.46 4.052c0 1.287.458 1.93 1.374 1.93.457 0 .807-.173 1.05-.52.246-.348.368-.847.368-1.499C7.252 4.654 6.805 4 5.91 4c-.471 0-.831.175-1.08.526-.247.35-.37.858-.37 1.526zm.862-.022c0-.922.183-1.383.55-1.383.344 0 .516.448.516 1.343s-.176 1.343-.527 1.343c-.36 0-.54-.434-.54-1.303zm3.187 1.886h2.435v-.672h-.792V4l-1.665.336v.687l.82-.177v2.398h-.798v.672zm-1.337 5H4.736v-.672h.798V9.846l-.82.177v-.687L6.38 9v3.244h.792v.671zm1.035-1.931c0 1.287.458 1.93 1.375 1.93.457 0 .807-.173 1.05-.52.245-.348.368-.847.368-1.499 0-1.309-.448-1.963-1.343-1.963-.47 0-.83.175-1.08.526-.246.35-.37.858-.37 1.526zm.862-.022c0-.922.184-1.383.55-1.383.344 0 .516.448.516 1.343s-.175 1.343-.526 1.343c-.36 0-.54-.434-.54-1.303z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFileCode;
impl IconShape for VsFileCode {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.57 1.14l3.28 3.3.15.36v9.7l-.5.5h-11l-.5-.5v-13l.5-.5h7.72l.35.14zM10 5h3l-3-3v3zM3 2v12h10V6H9.5L9 5.5V2H3zm2.062 7.533l1.817-1.828L6.17 7 4 9.179v.707l2.171 2.174.707-.707-1.816-1.82zM8.8 7.714l.7-.709 2.189 2.175v.709L9.5 12.062l-.705-.709 1.831-1.82L8.8 7.714z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFileMedia;
impl IconShape for VsFileMedia {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 2h6v3.5l.5.5H12v1h1V4.8l-.15-.36-3.28-3.3L9.22 1H1.5l-.5.5v13l.5.5H5v-1H2V2zm7 0l3 3H9V2zm5.5 6h-8l-.5.5v6l.5.5h8l.5-.5v-6l-.5-.5zM14 9v4l-1.63-1.6h-.71l-1.16 1.17-2.13-2.13h-.71L7 11.1V9h7zm-2.8 4.27l.81-.81L13.55 14h-1.62l-.73-.73zM7 14v-1.49l1-1L10.52 14H7zm5.5-3.5a.5.5 0 1 0 0-1 .5.5 0 0 0 0 1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFilePdf;
impl IconShape for VsFilePdf {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13.85 4.44l-3.28-3.3-.35-.14H2.5l-.5.5V7h1V2h6v3.5l.5.5H13v1h1V4.8l-.15-.36zM10 5V2l3 3h-3zM2.5 8l-.5.5v6l.5.5h11l.5-.5v-6l-.5-.5h-11zM13 13v1H3V9h10v4zm-8-1h-.32v1H4v-3h1.06c.75 0 1.13.36 1.13 1a.94.94 0 0 1-.32.72A1.33 1.33 0 0 1 5 12zm-.06-1.45h-.26v.93h.26c.36 0 .54-.16.54-.47 0-.31-.18-.46-.54-.46zM9 12.58a1.48 1.48 0 0 0 .44-1.12c0-1-.53-1.46-1.6-1.46H6.78v3h1.06A1.6 1.6 0 0 0 9 12.58zm-1.55-.13v-1.9h.33a.94.94 0 0 1 .7.25.91.91 0 0 1 .25.67 1 1 0 0 1-.25.72.94.94 0 0 1-.69.26h-.34zm4.45-.61h-.97V13h-.68v-3h1.74v.55h-1.06v.74h.97v.55z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFileSubmodule;
impl IconShape for VsFileSubmodule {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 11h1V6.99H2V11zm1-5.01V5.5l.5-.5h4.43l.43.25.43.75h5.71l.5.5v8l-.5.5h-11l-.5-.5V12H1.5l-.5-.5v-9l.5-.5h4.42l.44.25.43.75h5.71l.5.5V6l-1-.03V4H6.5l-.43-.25L5.64 3H2v2.99h1zm5.07.76L7.64 6H4v3h3.15l.41-.74L8 8h6V7H8.5l-.43-.25zM7.45 10H4v4h10V9H8.3l-.41.74-.44.26z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFileSymlinkDirectory;
impl IconShape for VsFileSymlinkDirectory {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.71 3h6.79l.51.5v10l-.5.5h-13l-.5-.5v-11l.5-.5h5l.35.15.85.85zm6.28 10v-1.51l.01-4v-1.5H7.7l-.86.86-.35.15H2v6h11.99zm-6.5-8h6.5l.01-.99H7.5l-.36-.15-.85-.85H2v3h4.28l.86-.86.35-.15zm2.29 4.07L8.42 7.7l.74-.69 2.22 2.22v.71l-2.29 2.21-.7-.72 1.4-1.35H8.42a2 2 0 0 0-1.35.61A1.8 1.8 0 0 0 6.54 12h-1a2.76 2.76 0 0 1 .81-2 3 3 0 0 1 2-.93h1.43z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFileSymlinkFile;
impl IconShape for VsFileSymlinkFile {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.57 1.14l3.28 3.3.15.36v9.7l-.5.5H10v-1h3V6H9.5L9 5.5V2H3v4H2V1.5l.5-.5h7.72l.35.14zM10 5h3l-3-3v3zM8.5 7h-7l-.5.5v7l.5.5h7l.5-.5v-7L8.5 7zM8 14H2V8h6v6zM7 9.5v3H6v-1.793l-2.646 2.647-.708-.708L5.293 10H3.53V9H6.5l.5.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFileZip;
impl IconShape for VsFileZip {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.5 1h11l.5.5v5l-.15.35-.85.86v6.79l-.5.5h-10l-.5-.5v-13l.5-.5zM6 2H5v2h1V2zm0 12h4V7.68l-.85-.85L9 6.47V2H7v2.5l-.5.5H6v1H5V5h-.5L4 4.5V2H3v12h2v-1h1v1zm0-2v1h1v-1H6zm0-1v1H5v-1h1zm0-1h1v1H6v-1zm0-1v1H5V9h1zm0-1h1v1H6V8zm0-1v1H5V7h1zm0 0h1V6H6v1zm6.15.15l.85-.86V2h-3v4.27l.85.85.15.35V14h1V7.5l.15-.35z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFile;
impl IconShape for VsFile {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13.71 4.29l-3-3L10 1H4L3 2v12l1 1h9l1-1V5l-.29-.71zM13 14H4V2h5v4h4v8zm-3-9V2l3 3h-3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFiles;
impl IconShape for VsFiles {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.5 0h-9L7 1.5V6H2.5L1 7.5v15.07L2.5 24h12.07L16 22.57V18h4.7l1.3-1.43V4.5L17.5 0zm0 2.12l2.38 2.38H17.5V2.12zm-3 20.38h-12v-15H7v9.07L8.5 18h6v4.5zm6-6h-12v-15H16V6h4.5v10.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFilterFilled;
impl IconShape for VsFilterFilled {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M15 2v1.67l-5 4.759V14H6V8.429l-5-4.76V2h14z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFilter;
impl IconShape for VsFilter {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M15 2v1.67l-5 4.759V14H6V8.429l-5-4.76V2h14zM7 8v5h2V8l5-4.76V3H2v.24L7 8z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFlag;
impl IconShape for VsFlag {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 9V3H12.0284L10.0931 5.70938C9.96896 5.88323 9.96896 6.11677 10.0931 6.29062L12.0284 9H4ZM4 10H13C13.4067 10 13.6432 9.54032 13.4069 9.20938L11.1145 6L13.4069 2.79062C13.6432 2.45968 13.4067 2 13 2H3.5C3.22386 2 3 2.22386 3 2.5V13.5C3 13.7761 3.22386 14 3.5 14C3.77614 14 4 13.7761 4 13.5V10Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFlame;
impl IconShape for VsFlame {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.13 15l-.53-.77a1.85 1.85 0 0 0-.28-2.54 3.51 3.51 0 0 1-1.19-2c-1.56 2.23-.75 3.46 0 4.55l-.55.76A4.4 4.4 0 0 1 3 10.46S2.79 8.3 5.28 6.19c0 0 2.82-2.61 1.84-4.54L7.83 1a6.57 6.57 0 0 1 2.61 6.94 2.57 2.57 0 0 0 .56-.81l.87-.07c.07.12 1.84 2.93.89 5.3A4.72 4.72 0 0 1 9.13 15zm-2-6.95l.87.39a3 3 0 0 0 .92 2.48 2.64 2.64 0 0 1 1 2.8A3.241 3.241 0 0 0 11.8 12a4.87 4.87 0 0 0-.41-3.63 1.85 1.85 0 0 1-1.84.86l-.35-.68a5.31 5.31 0 0 0-.89-5.8C8.17 4.87 6 6.83 5.93 6.94 3.86 8.7 4 10.33 4 10.4a3.47 3.47 0 0 0 1.59 3.14C5 12.14 5 10.46 7.16 8.05h-.03z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFoldDown;
impl IconShape for VsFoldDown {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.207 1.707L13.5 1l-6 6-6-6-.707.707 6.353 6.354h.708l6.353-6.354zm0 6L13.5 7l-6 6-6-6-.707.707 6.353 6.354h.708l6.353-6.354z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFoldUp;
impl IconShape for VsFoldUp {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1 7.4l.7.7 6-6 6 6 .7-.7L8.1 1h-.7L1 7.4zm0 6l.7.7 6-6 6 6 .7-.7L8.1 7h-.7L1 13.4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFold;
impl IconShape for VsFold {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.952 2.381L7.976 6.357 4 2.381 3.38 3l4.286 4.285h.619L12.57 3l-.618-.619zM3.904 14l4.072-4.072L12.047 14l.62-.619L8.284 9h-.619l-4.381 4.381.619.619z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFolderActive;
impl IconShape for VsFolderActive {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.71 3h6.79l.51.5v4.507A4.997 4.997 0 0 0 14 7.416V5.99H7.69l-.86.86-.35.15H1.99v6H7.1c.07.348.177.682.316 1H1.51l-.5-.5v-11l.5-.5h5l.35.15.85.85zm-.22 2h6.5l.01-.99H7.5l-.36-.15-.85-.85H2v3h4.28l.86-.86.35-.15z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M9.778 8.674a4 4 0 1 1 4.444 6.652 4 4 0 0 1-4.444-6.652zm2.13 4.99l2.387-3.182-.8-.6-2.077 2.769-1.301-1.041-.625.78 1.704 1.364.713-.09z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFolderLibrary;
impl IconShape for VsFolderLibrary {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.70996 3H14.5L15.01 3.5V7H14V5.98999H7.68994L6.82996 6.84998L6.47998 7H1.98999V7.48999V11.49V13H7V14H1.51001L1.01001 13.5V6.5V2.5L1.51001 2H6.51001L6.85999 2.15002L7.70996 3ZM7.48999 5H13.99L14 4.01001H7.5L7.14001 3.85999L6.29004 3.01001H2V6.01001H6.28003L7.14001 5.15002L7.48999 5Z",
                fill_rule: "evenodd",
            }
            rect {
                height: "6",
                width: "1",
                x: "8",
                y: "8",
            }
            rect {
                height: "6",
                width: "1",
                x: "10",
                y: "8",
            }
            rect {
                height: "6",
                transform: "rotate(-20 12.0041 8.35193)",
                width: "1",
                x: "12.0041",
                y: "8.35193",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFolderOpened;
impl IconShape for VsFolderOpened {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.5 14h11l.48-.37 2.63-7-.48-.63H14V3.5l-.5-.5H7.71l-.86-.85L6.5 2h-5l-.5.5v11l.5.5zM2 3h4.29l.86.85.35.15H13v2H8.5l-.35.15-.86.85H3.5l-.47.34-1 3.08L2 3zm10.13 10H2.19l1.67-5H7.5l.35-.15.86-.85h5.79l-2.37 6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsFolder;
impl IconShape for VsFolder {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.5 3H7.71l-.85-.85L6.51 2h-5l-.5.5v11l.5.5h13l.5-.5v-10L14.5 3zm-.51 8.49V13h-12V7h4.49l.35-.15.86-.86H14v1.5l-.01 4zm0-6.49h-6.5l-.35.15-.86.86H2v-3h4.29l.85.85.36.15H14l-.01.99z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGame;
impl IconShape for VsGame {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 3H12C14.2091 3 16 4.79086 16 7V10C16 12.2091 14.2091 14 12 14H4C1.79086 14 0 12.2091 0 10V7C0 4.79086 1.79086 3 4 3ZM4 4C2.34315 4 1 5.34315 1 7V10C1 11.6569 2.34315 13 4 13H12C13.6569 13 15 11.6569 15 10V7C15 5.34315 13.6569 4 12 4H4Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M5.5 6C5.22386 6 5 6.22386 5 6.5V8H3.5C3.22386 8 3 8.22386 3 8.5C3 8.77614 3.22386 9 3.5 9H5V10.5C5 10.7761 5.22386 11 5.5 11C5.77614 11 6 10.7761 6 10.5V9H7.5C7.77614 9 8 8.77614 8 8.5C8 8.22386 7.77614 8 7.5 8H6V6.5C6 6.22386 5.77614 6 5.5 6Z",
            }
            path {
                d: "M13 7C13 7.55228 12.5523 8 12 8C11.4477 8 11 7.55228 11 7C11 6.44772 11.4477 6 12 6C12.5523 6 13 6.44772 13 7Z",
            }
            path {
                d: "M12 10C12 10.5523 11.5523 11 11 11C10.4477 11 10 10.5523 10 10C10 9.44772 10.4477 9 11 9C11.5523 9 12 9.44772 12 10Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGear;
impl IconShape for VsGear {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.1 4.4L8.6 2H7.4l-.5 2.4-.7.3-2-1.3-.9.8 1.3 2-.2.7-2.4.5v1.2l2.4.5.3.8-1.3 2 .8.8 2-1.3.8.3.4 2.3h1.2l.5-2.4.8-.3 2 1.3.8-.8-1.3-2 .3-.8 2.3-.4V7.4l-2.4-.5-.3-.8 1.3-2-.8-.8-2 1.3-.7-.2zM9.4 1l.5 2.4L12 2.1l2 2-1.4 2.1 2.4.4v2.8l-2.4.5L14 12l-2 2-2.1-1.4-.5 2.4H6.6l-.5-2.4L4 13.9l-2-2 1.4-2.1L1 9.4V6.6l2.4-.5L2.1 4l2-2 2.1 1.4.4-2.4h2.8zm.6 7c0 1.1-.9 2-2 2s-2-.9-2-2 .9-2 2-2 2 .9 2 2zM8 9c.6 0 1-.4 1-1s-.4-1-1-1-1 .4-1 1 .4 1 1 1z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGift;
impl IconShape for VsGift {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.5 4h-1.6c.1-.4.1-.8.1-1.2-.1-.3-.2-.6-.4-.9-.2-.3-.4-.5-.7-.6-.3-.1-.6-.3-.9-.3-.3 0-.6 0-.9.2-.7.2-1.2.7-1.6 1.3-.4-.6-.9-1.1-1.6-1.3-.3-.1-.6-.2-.9-.2-.3 0-.6.1-.9.3-.3.1-.5.3-.7.6-.2.2-.3.6-.4.9 0 .4 0 .8.1 1.2H1.5l-.5.5v9l.5.5h12l.5-.5v-9l-.5-.5zM7 13H2V5h5v8zm0-9H4v-.2c-.1-.3-.1-.5-.1-.8.1-.2.1-.4.3-.5.1-.2.3-.3.5-.4.1-.1.3-.1.5-.1s.4 0 .6.1c.3.1.6.3.8.6.2.3.4.6.4 1V4zm1-.3c0-.4.2-.7.4-1 .2-.3.5-.5.8-.6.2-.1.4-.1.6-.1.2 0 .4 0 .6.1.2.1.3.2.5.4.1.1.1.3.2.5 0 .3 0 .5-.1.8 0 .1 0 .1-.1.2H8v-.3zm5 9.3H8V5h5v8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGistSecret;
impl IconShape for VsGistSecret {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 14h4v.91l.09.09H2.5l-.5-.5v-13l.5-.5h7.72l.35.14 3.28 3.3.15.36v2.54a3.1 3.1 0 0 0-1-.94V6H9.5L9 5.5V2H3v12zm10-9l-3-3v3h3zm.5 4v1h1l.5.5v4l-.5.5h-6l-.5-.5v-4l.5-.5h1V9a2 2 0 0 1 4 0zm-2.707-.707A1 1 0 0 0 10.5 9v1h2V9a1 1 0 0 0-1.707-.707zM9 11v3h5v-3H9z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGist;
impl IconShape for VsGist {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.57 1.14l3.28 3.3.15.36v9.7l-.5.5h-11l-.5-.5v-13l.5-.5h7.72l.35.14zM10 5h3l-3-3v3zM3 2v12h10V6H9.5L9 5.5V2H3zm2.062 7.533l1.817-1.828L6.17 7 4 9.179v.707l2.171 2.174.707-.707-1.816-1.82zM8.8 7.714l.7-.709 2.189 2.175v.709L9.5 12.062l-.705-.709 1.831-1.82L8.8 7.714z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGitCommit;
impl IconShape for VsGitCommit {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.5 0H8.5V4.03095C10.4732 4.277 12 5.96019 12 8C12 10.0398 10.4732 11.723 8.5 11.9691V16H7.5V11.9691C5.52684 11.723 4 10.0398 4 8C4 5.96019 5.52684 4.277 7.5 4.03095V0ZM8 10.6C9.43594 10.6 10.6 9.43594 10.6 8C10.6 6.56406 9.43594 5.4 8 5.4C6.56406 5.4 5.4 6.56406 5.4 8C5.4 9.43594 6.56406 10.6 8 10.6Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGitCompare;
impl IconShape for VsGitCompare {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.389 12.99l-1.27-1.27.67-.7 2.13 2.13v.7l-2.13 2.13-.71-.71L7.349 14h-1.85a2.49 2.49 0 0 1-2.5-2.5V5.95a2.59 2.59 0 0 1-1.27-.68 2.52 2.52 0 0 1-.54-2.73A2.5 2.5 0 0 1 3.499 1a2.45 2.45 0 0 1 1 .19 2.48 2.48 0 0 1 1.35 1.35c.133.317.197.658.19 1a2.5 2.5 0 0 1-2 2.45v5.5a1.5 1.5 0 0 0 1.5 1.5h1.85zm-4.68-8.25a1.5 1.5 0 0 0 2.08-2.08 1.55 1.55 0 0 0-.68-.56 1.49 1.49 0 0 0-.86-.08 1.49 1.49 0 0 0-1.18 1.18 1.49 1.49 0 0 0 .08.86c.117.277.311.513.56.68zm10.33 6.3c.48.098.922.335 1.27.68a2.51 2.51 0 0 1 .31 3.159 2.5 2.5 0 1 1-3.47-3.468c.269-.182.571-.308.89-.37V5.49a1.5 1.5 0 0 0-1.5-1.5h-1.85l1.27 1.27-.71.71-2.13-2.13v-.7l2.13-2.13.71.71-1.27 1.27h1.85a2.49 2.49 0 0 1 2.5 2.5v5.55zm-.351 3.943a1.5 1.5 0 0 0 1.1-2.322 1.55 1.55 0 0 0-.68-.56 1.49 1.49 0 0 0-.859-.08 1.49 1.49 0 0 0-1.18 1.18 1.49 1.49 0 0 0 .08.86 1.5 1.5 0 0 0 1.539.922z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGitFetch;
impl IconShape for VsGitFetch {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 2H8V3H9V2Z",
            }
            path {
                d: "M8.14644 14L3.14645 9.00001L3.85356 8.2929L8 12.4393L8 11L9 11L9 12.4393L13.1464 8.2929L13.8535 9.00001L8.85354 14H8.14644Z",
            }
            path {
                d: "M8 5H9V6H8V5Z",
            }
            path {
                d: "M9 8H8V9H9V8Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGitMerge;
impl IconShape for VsGitMerge {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.273 7.73a2.51 2.51 0 0 0-3.159-.31 2.5 2.5 0 0 0-.921 1.12 2.23 2.23 0 0 0-.13.44 4.52 4.52 0 0 1-4-4 2.23 2.23 0 0 0 .44-.13 2.5 2.5 0 0 0 1.54-2.31 2.45 2.45 0 0 0-.19-1A2.48 2.48 0 0 0 5.503.19a2.45 2.45 0 0 0-1-.19 2.5 2.5 0 0 0-2.31 1.54 2.52 2.52 0 0 0 .54 2.73c.35.343.79.579 1.27.68v5.1a2.411 2.411 0 0 0-.89.37 2.5 2.5 0 1 0 3.47 3.468 2.5 2.5 0 0 0 .42-1.387 2.45 2.45 0 0 0-.19-1 2.48 2.48 0 0 0-1.81-1.49v-2.4a5.52 5.52 0 0 0 2 1.73 5.65 5.65 0 0 0 2.09.6 2.5 2.5 0 0 0 4.95-.49 2.51 2.51 0 0 0-.77-1.72zm-8.2 3.38c.276.117.512.312.68.56a1.5 1.5 0 0 1-2.08 2.08 1.55 1.55 0 0 1-.56-.68 1.49 1.49 0 0 1-.08-.86 1.49 1.49 0 0 1 1.18-1.18 1.49 1.49 0 0 1 .86.08zM4.503 4a1.5 1.5 0 0 1-1.39-.93 1.49 1.49 0 0 1-.08-.86 1.49 1.49 0 0 1 1.18-1.18 1.49 1.49 0 0 1 .86.08A1.5 1.5 0 0 1 4.503 4zm8.06 6.56a1.5 1.5 0 0 1-2.45-.49 1.49 1.49 0 0 1-.08-.86 1.49 1.49 0 0 1 1.18-1.18 1.49 1.49 0 0 1 .86.08 1.499 1.499 0 0 1 .49 2.45z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGitPullRequestClosed;
impl IconShape for VsGitPullRequestClosed {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.61594 4.92769C5.34304 5.33899 4.95319 5.66062 4.49705 5.8497C4.34891 5.91013 4.03897 5.9881 4.03897 5.9881V10.9958C4.19686 11.027 4.35086 11.0738 4.499 11.1362C4.95513 11.3272 5.34304 11.6469 5.61789 12.0582C5.89079 12.4695 6.03699 12.9529 6.03699 13.4461C6.04478 13.7891 5.98046 14.1303 5.84791 14.446C5.72315 14.7482 5.53992 15.023 5.30796 15.255C5.07794 15.487 4.80114 15.6702 4.499 15.7949C4.18322 15.9275 3.84209 15.9918 3.49902 15.984C3.00585 15.986 2.52243 15.8398 2.11113 15.5649C1.69983 15.292 1.3782 14.9022 1.18912 14.446C1.00198 13.988 0.953253 13.485 1.04877 12.9997C1.14428 12.5143 1.38015 12.0679 1.72907 11.717C2.07799 11.374 2.51853 11.1381 2.99805 11.0367V5.94911C2.52048 5.8458 2.07994 5.61189 1.72907 5.26881C1.38015 4.91794 1.14428 4.47155 1.04877 3.98618C0.951304 3.50081 1.00004 2.99789 1.18912 2.53981C1.3782 2.08368 1.69983 1.69382 2.11113 1.42092C2.52048 1.14607 3.0039 0.999877 3.49902 0.999877C3.84014 0.99403 4.18127 1.05836 4.49705 1.18896C4.79919 1.31371 5.07404 1.49695 5.30601 1.72891C5.53797 1.96087 5.7212 2.23767 5.84596 2.53981C5.97851 2.8556 6.04284 3.19672 6.03504 3.5398C6.03699 4.03296 5.89079 4.51639 5.61594 4.92769ZM4.85962 12.7892C4.73097 12.5494 4.53994 12.3486 4.30797 12.2102C4.07601 12.0699 3.80896 11.9958 3.538 11.9997C3.24171 11.9997 2.95322 12.0855 2.70761 12.2492C2.46005 12.4168 2.26512 12.6527 2.14816 12.9295C2.03706 13.2024 2.00977 13.5006 2.06824 13.7891C2.12477 14.0796 2.26707 14.3486 2.47759 14.5591C2.68812 14.7696 2.95517 14.9119 3.24756 14.9685C3.53606 15.0269 3.8343 14.9996 4.1072 14.8885C4.38399 14.7716 4.61986 14.5766 4.7875 14.3291C4.93759 14.103 5.02336 13.8398 5.037 13.5689C5.0487 13.2979 4.98827 13.0289 4.85962 12.7892ZM2.70761 4.74056C2.95517 4.90235 3.24366 4.99006 3.538 4.99006C3.80896 4.99006 4.07601 4.91599 4.30797 4.77954C4.53994 4.63919 4.73097 4.44037 4.85962 4.2006C4.98827 3.96084 5.05065 3.69184 5.037 3.42089C5.02336 3.14994 4.93759 2.88679 4.7875 2.66067C4.61986 2.41311 4.38399 2.21818 4.1072 2.10122C3.8343 1.99011 3.53606 1.96282 3.24756 2.0213C2.95712 2.07783 2.68812 2.22013 2.47759 2.43065C2.26707 2.64118 2.12477 2.90823 2.06824 3.20062C2.00977 3.48911 2.03706 3.78735 2.14816 4.06025C2.26512 4.33705 2.46005 4.57292 2.70761 4.74056ZM13.0368 11.0368C13.5164 11.1342 13.9588 11.372 14.3058 11.7171C14.7717 12.1868 15.0348 12.8243 15.0309 13.4831C15.0329 13.9763 14.8867 14.4597 14.6119 14.871C14.339 15.2823 13.9491 15.6039 13.493 15.793C13.0368 15.984 12.532 16.0347 12.0466 15.9392C11.5612 15.8437 11.1148 15.6059 10.764 15.255C10.415 14.9041 10.1753 14.4578 10.0798 13.9724C9.98425 13.487 10.0349 12.9841 10.226 12.526C10.4189 12.0738 10.7386 11.6839 11.146 11.4071C11.4131 11.2239 11.7172 11.0991 12.0349 11.0368V7.4891H13.0368V11.0368ZM13.5943 14.5455C13.8399 14.3018 13.992 13.9802 14.0271 13.6352C14.0622 13.2921 13.9764 12.9451 13.7854 12.6566C13.6177 12.4091 13.3819 12.2141 13.1051 12.0972C12.8322 11.9861 12.5339 11.9588 12.2454 12.0173C11.955 12.0738 11.686 12.2161 11.4755 12.4266C11.2649 12.6371 11.1226 12.9042 11.0661 13.1966C11.0076 13.4851 11.0349 13.7833 11.146 14.0562C11.263 14.333 11.4579 14.5689 11.7055 14.7365C11.994 14.9275 12.339 15.0133 12.684 14.9782C13.0271 14.9431 13.3507 14.7911 13.5943 14.5455Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M11.6876 3.40036L10 5.088L10.7071 5.7951L12.3947 4.10747L14.0824 5.7951L14.7895 5.088L13.1019 3.40036L14.7895 1.71272L14.0824 1.00562L12.3947 2.69325L10.7071 1.00562L10 1.71272L11.6876 3.40036Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGitPullRequestCreate;
impl IconShape for VsGitPullRequestCreate {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.616 4.928a2.487 2.487 0 0 1-1.119.922c-.148.06-.458.138-.458.138v5.008a2.51 2.51 0 0 1 1.579 1.062c.273.412.419.895.419 1.388.008.343-.057.684-.19 1A2.485 2.485 0 0 1 3.5 15.984a2.482 2.482 0 0 1-1.388-.419A2.487 2.487 0 0 1 1.05 13c.095-.486.331-.932.68-1.283.349-.343.79-.579 1.269-.68V5.949a2.6 2.6 0 0 1-1.269-.68 2.503 2.503 0 0 1-.68-1.283 2.487 2.487 0 0 1 1.06-2.565A2.49 2.49 0 0 1 3.5 1a2.504 2.504 0 0 1 1.807.729 2.493 2.493 0 0 1 .729 1.81c.002.494-.144.978-.42 1.389zm-.756 7.861a1.5 1.5 0 0 0-.552-.579 1.45 1.45 0 0 0-.77-.21 1.495 1.495 0 0 0-1.47 1.79 1.493 1.493 0 0 0 1.18 1.179c.288.058.586.03.86-.08.276-.117.512-.312.68-.56.15-.226.235-.49.249-.76a1.51 1.51 0 0 0-.177-.78zM2.708 4.741c.247.161.536.25.83.25.271 0 .538-.075.77-.211a1.514 1.514 0 0 0 .729-1.359 1.513 1.513 0 0 0-.25-.76 1.551 1.551 0 0 0-.68-.56 1.49 1.49 0 0 0-.86-.08 1.494 1.494 0 0 0-1.179 1.18c-.058.288-.03.586.08.86.117.276.312.512.56.68zM13.037 7h-1.002V5.49a1.5 1.5 0 0 0-1.5-1.5H8.687l1.269 1.27-.71.709L7.117 3.84v-.7l2.13-2.13.71.711-1.269 1.27h1.85a2.484 2.484 0 0 1 2.312 1.541c.125.302.189.628.187.957V7zM13 16h-1v-3H9v-1h3V9h1v3h3v1h-3v3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGitPullRequestDraft;
impl IconShape for VsGitPullRequestDraft {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.49705 5.8497C4.95319 5.66062 5.34304 5.33899 5.61594 4.92769C5.89079 4.51639 6.03699 4.03296 6.03504 3.5398C6.04284 3.19672 5.97851 2.8556 5.84596 2.53981C5.7212 2.23767 5.53797 1.96087 5.30601 1.72891C5.07404 1.49695 4.79919 1.31371 4.49705 1.18896C4.18127 1.05836 3.84014 0.99403 3.49902 0.999877C3.0039 0.999877 2.52048 1.14607 2.11113 1.42092C1.69983 1.69382 1.3782 2.08368 1.18912 2.53981C1.00004 2.99789 0.951304 3.50081 1.04877 3.98618C1.14428 4.47155 1.38015 4.91794 1.72907 5.26881C2.07994 5.61189 2.52048 5.8458 2.99805 5.94911V11.0367C2.51853 11.1381 2.07799 11.374 1.72907 11.717C1.38015 12.0679 1.14428 12.5143 1.04877 12.9997C0.953253 13.485 1.00198 13.988 1.18912 14.446C1.3782 14.9022 1.69983 15.292 2.11113 15.5649C2.52243 15.8398 3.00585 15.986 3.49902 15.984C3.84209 15.9918 4.18322 15.9275 4.499 15.7949C4.80114 15.6702 5.07794 15.487 5.30796 15.255C5.53992 15.023 5.72315 14.7482 5.84791 14.446C5.98046 14.1303 6.04478 13.7891 6.03699 13.4461C6.03699 12.9529 5.89079 12.4695 5.61789 12.0582C5.34304 11.6469 4.95513 11.3272 4.499 11.1362C4.35086 11.0738 4.19686 11.027 4.03897 10.9958V5.9881C4.03897 5.9881 4.34891 5.91013 4.49705 5.8497ZM4.30797 12.2102C4.53994 12.3486 4.73097 12.5494 4.85962 12.7892C4.98827 13.0289 5.0487 13.2979 5.037 13.5689C5.02336 13.8398 4.93759 14.103 4.7875 14.3291C4.61986 14.5766 4.38399 14.7716 4.1072 14.8885C3.8343 14.9996 3.53606 15.0269 3.24756 14.9685C2.95517 14.9119 2.68812 14.7696 2.47759 14.5591C2.26707 14.3486 2.12477 14.0796 2.06824 13.7891C2.00977 13.5006 2.03706 13.2024 2.14816 12.9295C2.26512 12.6527 2.46005 12.4168 2.70761 12.2492C2.95322 12.0855 3.24171 11.9997 3.538 11.9997C3.80896 11.9958 4.07601 12.0699 4.30797 12.2102ZM3.538 4.99006C3.24366 4.99006 2.95517 4.90235 2.70761 4.74056C2.46005 4.57292 2.26512 4.33705 2.14816 4.06025C2.03706 3.78735 2.00977 3.48911 2.06824 3.20062C2.12477 2.90823 2.26707 2.64118 2.47759 2.43065C2.68812 2.22013 2.95712 2.07783 3.24756 2.0213C3.53606 1.96282 3.8343 1.99011 4.1072 2.10122C4.38399 2.21818 4.61986 2.41311 4.7875 2.66067C4.93759 2.88679 5.02336 3.14994 5.037 3.42089C5.05065 3.69184 4.98827 3.96084 4.85962 4.2006C4.73097 4.44037 4.53994 4.63919 4.30797 4.77954C4.07601 4.91599 3.80896 4.99006 3.538 4.99006Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M15.0543 13.5C15.0543 14.8807 13.935 16 12.5543 16C11.1736 16 10.0543 14.8807 10.0543 13.5C10.0543 12.1193 11.1736 11 12.5543 11C13.935 11 15.0543 12.1193 15.0543 13.5ZM12.5543 15C13.3827 15 14.0543 14.3284 14.0543 13.5C14.0543 12.6716 13.3827 12 12.5543 12C11.7258 12 11.0543 12.6716 11.0543 13.5C11.0543 14.3284 11.7258 15 12.5543 15Z",
                fill_rule: "evenodd",
            }
            circle {
                cx: "12.5543",
                cy: "7.75073",
                r: "1",
            }
            circle {
                cx: "12.5543",
                cy: "3.50146",
                r: "1",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGitPullRequestGoToChanges;
impl IconShape for VsGitPullRequestGoToChanges {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.99994 10V14L3.99994 15H12.9999L13.9999 14V5L13.7099 4.29L10.7099 1.29L9.99994 1H8V2H9.99994L12.9999 5V14H3.99994V10H2.99994ZM11 6H9V4H8V6H6V7H8V9H9V7H11V6ZM6 11H11V12H6V11Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M7.06065 3.85356L4.91421 6L4.2071 5.29289L5.49999 4H2.5C2.10218 4 1.72064 4.15804 1.43934 4.43934C1.15804 4.72065 1 5.10218 1 5.5C1 5.89783 1.15804 6.27936 1.43934 6.56066C1.72064 6.84197 2.10218 7 2.5 7H3V8H2.5C1.83696 8 1.20107 7.73661 0.732233 7.26777C0.263392 6.79893 0 6.16305 0 5.5C0 4.83696 0.263392 4.20108 0.732233 3.73224C1.20107 3.2634 1.83696 3 2.5 3H5.49999L4.2071 1.70711L4.91421 1L7.06065 3.14645L7.06065 3.85356Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGitPullRequestNewChanges;
impl IconShape for VsGitPullRequestNewChanges {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13.7099 4.29L10.7099 1.29L9.99994 1H3.99994L2.99994 2V14L3.99994 15H9.35418C9.03018 14.714 8.75287 14.3764 8.53513 14H3.99994V2H9.99994L12.9999 5V8.126C13.3547 8.21731 13.6904 8.35606 13.9999 8.53509V5L13.7099 4.29ZM8.12602 11H6V12H8C8 11.6547 8.04375 11.3196 8.12602 11ZM6 6H8V4H9V6H11V7H9V9H8V7H6V6Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M12 9C11.4067 9 10.8266 9.17595 10.3333 9.50559C9.83994 9.83524 9.45543 10.3038 9.22836 10.8519C9.0013 11.4001 8.94189 12.0033 9.05765 12.5853C9.1734 13.1672 9.45912 13.7018 9.87868 14.1213C10.2982 14.5409 10.8328 14.8266 11.4147 14.9424C11.9967 15.0581 12.5999 14.9987 13.1481 14.7716C13.6962 14.5446 14.1648 14.1601 14.4944 13.6667C14.8241 13.1734 15 12.5933 15 12C14.999 11.2047 14.6826 10.4422 14.1202 9.87976C13.5578 9.31736 12.7954 9.00098 12 9Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGitPullRequest;
impl IconShape for VsGitPullRequest {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.616 4.928a2.487 2.487 0 0 1-1.119.922c-.148.06-.458.138-.458.138v5.008a2.51 2.51 0 0 1 1.579 1.062c.273.412.419.895.419 1.388.008.343-.057.684-.19 1A2.485 2.485 0 0 1 3.5 15.984a2.482 2.482 0 0 1-1.388-.419A2.487 2.487 0 0 1 1.05 13c.095-.486.331-.932.68-1.283.349-.343.79-.579 1.269-.68V5.949a2.6 2.6 0 0 1-1.269-.68 2.503 2.503 0 0 1-.68-1.283 2.487 2.487 0 0 1 1.06-2.565A2.49 2.49 0 0 1 3.5 1a2.504 2.504 0 0 1 1.807.729 2.493 2.493 0 0 1 .729 1.81c.002.494-.144.978-.42 1.389zm-.756 7.861a1.5 1.5 0 0 0-.552-.579 1.45 1.45 0 0 0-.77-.21 1.495 1.495 0 0 0-1.47 1.79 1.493 1.493 0 0 0 1.18 1.179c.288.058.586.03.86-.08.276-.117.512-.312.68-.56.15-.226.235-.49.249-.76a1.51 1.51 0 0 0-.177-.78zM2.708 4.741c.247.161.536.25.83.25.271 0 .538-.075.77-.211a1.514 1.514 0 0 0 .729-1.359 1.513 1.513 0 0 0-.25-.76 1.551 1.551 0 0 0-.68-.56 1.49 1.49 0 0 0-.86-.08 1.494 1.494 0 0 0-1.179 1.18c-.058.288-.03.586.08.86.117.276.312.512.56.68zm10.329 6.296c.48.097.922.335 1.269.68.466.47.729 1.107.725 1.766.002.493-.144.977-.42 1.388a2.499 2.499 0 0 1-4.532-.899 2.5 2.5 0 0 1 1.067-2.565c.267-.183.571-.308.889-.37V5.489a1.5 1.5 0 0 0-1.5-1.499H8.687l1.269 1.27-.71.709L7.117 3.84v-.7l2.13-2.13.71.711-1.269 1.27h1.85a2.484 2.484 0 0 1 2.312 1.541c.125.302.189.628.187.957v5.548zm.557 3.509a1.493 1.493 0 0 0 .191-1.89 1.552 1.552 0 0 0-.68-.559 1.49 1.49 0 0 0-.86-.08 1.493 1.493 0 0 0-1.179 1.18 1.49 1.49 0 0 0 .08.86 1.496 1.496 0 0 0 2.448.49z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGitStashApply;
impl IconShape for VsGitStashApply {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.00013 3.20711L7.00013 5H8.00013L8.00013 3.20711L10.6466 5.85356L11.3537 5.14645L7.85368 1.64645H7.14658L3.64658 5.14645L4.35368 5.85356L7.00013 3.20711Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M1.50012 9L1.00012 9.5V14.5L1.50012 15H13.5001L14.0001 14.5V9.5L13.5001 9H9.95012C9.71848 10.1411 8.7096 11 7.50012 11C6.29064 11 5.28177 10.1411 5.05013 9H1.50012ZM10.6633 10H13.0001V14H2.00012V10H4.33694C4.89867 11.1825 6.10392 12 7.50012 12C8.89632 12 10.1016 11.1825 10.6633 10Z",
            }
            rect {
                height: "1",
                width: "1",
                x: "7",
                y: "6",
            }
            rect {
                height: "1",
                width: "1",
                x: "7",
                y: "8",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGitStashPop;
impl IconShape for VsGitStashPop {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.00013 3.20711L7.00013 9H8.00013L8.00013 3.20711L10.6466 5.85356L11.3537 5.14645L7.85368 1.64645H7.14658L3.64658 5.14645L4.35368 5.85356L7.00013 3.20711Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M1.50012 9L1.00012 9.5V14.5L1.50012 15H13.5001L14.0001 14.5V9.5L13.5001 9H9.95012C9.71848 10.1411 8.7096 11 7.50012 11C6.29064 11 5.28177 10.1411 5.05013 9H1.50012ZM10.6633 10H13.0001V14H2.00012V10H4.33694C4.89867 11.1825 6.10392 12 7.50012 12C8.89632 12 10.1016 11.1825 10.6633 10Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGitStash;
impl IconShape for VsGitStash {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.00013 6.79289L7.00013 1H8.00013L8.00013 6.79289L10.6466 4.14644L11.3537 4.85355L7.85368 8.35355H7.14658L3.64658 4.85355L4.35368 4.14644L7.00013 6.79289Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M1.50012 9L1.00012 9.5V14.5L1.50012 15H13.5001L14.0001 14.5V9.5L13.5001 9H9.95012C9.71848 10.1411 8.7096 11 7.50012 11C6.29064 11 5.28177 10.1411 5.05013 9H1.50012ZM10.6633 10H13.0001V14H2.00012V10H4.33694C4.89867 11.1825 6.10392 12 7.50012 12C8.89632 12 10.1016 11.1825 10.6633 10Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGithubAction;
impl IconShape for VsGithubAction {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3.04 10h2.58l.65 1H2.54l-.5-.5v-9l.5-.5h12l.5.5v4.77l-1-1.75V2h-11v8zm5.54 1l-1.41 3.47h2.2L15 8.7 14.27 7h-1.63l.82-1.46L12.63 4H9.76l-.92.59-2.28 5L7.47 11h1.11zm1.18-6h2.87l-1.87 3h3.51l-5.76 5.84L10.2 10H7.47l2.29-5zM6.95 7H4.04V6H7.4l-.45 1zm-.9 2H4.04V8H6.5l-.45 1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGithubAlt;
impl IconShape for VsGithubAlt {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21.035 5.257c.91 1.092 1.364 2.366 1.364 3.822 0 5.277-3.002 6.824-5.823 7.279.364.637.455 1.365.455 2.093v3.73c0 .455-.273.728-.637.728a.718.718 0 0 1-.728-.728v-3.73a2.497 2.497 0 0 0-.728-2.093l.455-1.183c2.821-.364 5.733-1.274 5.733-6.187 0-1.183-.455-2.275-1.274-3.185l-.182-.727a4.04 4.04 0 0 0 .09-2.73c-.454.09-1.364.273-2.91 1.365l-.547.09a13.307 13.307 0 0 0-6.55 0l-.547-.09C7.57 2.71 6.66 2.437 6.204 2.437c-.273.91-.273 1.91.09 2.73l-.181.727c-.91.91-1.365 2.093-1.365 3.185 0 4.822 2.73 5.823 5.732 6.187l.364 1.183c-.546.546-.819 1.274-.728 2.002v3.821a.718.718 0 0 1-.728.728.718.718 0 0 1-.728-.728V20.18c-3.002.637-4.185-.91-5.095-2.092-.455-.546-.819-1.001-1.274-1.092-.09-.091-.364-.455-.273-.819.091-.364.455-.637.82-.455.91.182 1.455.91 2 1.547.82 1.092 1.639 2.092 4.095 1.547v-.364c-.09-.728.091-1.456.455-2.093-2.73-.546-5.914-2.093-5.914-7.279 0-1.456.455-2.73 1.365-3.822-.273-1.273-.182-2.638.273-3.73l.455-.364C5.749 1.073 7.023.8 9.66 2.437a13.673 13.673 0 0 1 6.642 0C18.851.708 20.216.98 20.398 1.072l.455.364c.455 1.274.546 2.548.182 3.821z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGithubInverted;
impl IconShape for VsGithubInverted {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.976 0A7.977 7.977 0 0 0 0 7.976c0 3.522 2.3 6.507 5.431 7.584.392.049.538-.196.538-.392v-1.37c-2.201.49-2.69-1.076-2.69-1.076-.343-.93-.881-1.175-.881-1.175-.734-.489.048-.489.048-.489.783.049 1.224.832 1.224.832.734 1.223 1.859.88 2.3.685.048-.538.293-.88.489-1.076-1.762-.196-3.621-.881-3.621-3.964 0-.88.293-1.566.832-2.153-.05-.147-.343-.978.098-2.055 0 0 .685-.196 2.201.832.636-.196 1.322-.245 2.007-.245s1.37.098 2.006.245c1.517-1.027 2.202-.832 2.202-.832.44 1.077.146 1.908.097 2.104a3.16 3.16 0 0 1 .832 2.153c0 3.083-1.86 3.719-3.62 3.915.293.244.538.733.538 1.467v2.202c0 .196.146.44.538.392A7.984 7.984 0 0 0 16 7.976C15.951 3.572 12.38 0 7.976 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGithubProject;
impl IconShape for VsGithubProject {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6.00012 13H7.00012L7.00012 7.00001L13.0001 7.00001V6.00001L7.00012 6.00001L7.00012 3H6.00012L6.00012 6.00001L3.00012 6.00001V7.00001H6.00012L6.00012 13Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M2.50012 2H13.5001L14.0001 2.5V13.5L13.5001 14H2.50012L2.00012 13.5V2.5L2.50012 2ZM3.00012 13H13.0001V3H3.00012V13Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGithub;
impl IconShape for VsGithub {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 0a12 12 0 1 0 0 24 12 12 0 0 0 0-24zm3.163 21.783h-.093a.513.513 0 0 1-.382-.14.513.513 0 0 1-.14-.372v-1.406c.006-.467.01-.94.01-1.416a3.693 3.693 0 0 0-.151-1.028 1.832 1.832 0 0 0-.542-.875 8.014 8.014 0 0 0 2.038-.471 4.051 4.051 0 0 0 1.466-.964c.407-.427.71-.943.885-1.506a6.77 6.77 0 0 0 .3-2.13 4.138 4.138 0 0 0-.26-1.476 3.892 3.892 0 0 0-.795-1.284 2.81 2.81 0 0 0 .162-.582c.033-.2.05-.402.05-.604 0-.26-.03-.52-.09-.773a5.309 5.309 0 0 0-.221-.763.293.293 0 0 0-.111-.02h-.11c-.23.002-.456.04-.674.111a5.34 5.34 0 0 0-.703.26 6.503 6.503 0 0 0-.661.343c-.215.127-.405.249-.573.362a9.578 9.578 0 0 0-5.143 0 13.507 13.507 0 0 0-.572-.362 6.022 6.022 0 0 0-.672-.342 4.516 4.516 0 0 0-.705-.261 2.203 2.203 0 0 0-.662-.111h-.11a.29.29 0 0 0-.11.02 5.844 5.844 0 0 0-.23.763c-.054.254-.08.513-.081.773 0 .202.017.404.051.604.033.199.086.394.16.582A3.888 3.888 0 0 0 5.702 10a4.142 4.142 0 0 0-.263 1.476 6.871 6.871 0 0 0 .292 2.12c.181.563.483 1.08.884 1.516.415.422.915.75 1.466.964.653.25 1.337.41 2.033.476a1.828 1.828 0 0 0-.452.633 2.99 2.99 0 0 0-.2.744 2.754 2.754 0 0 1-1.175.27 1.788 1.788 0 0 1-1.065-.3 2.904 2.904 0 0 1-.752-.824 3.1 3.1 0 0 0-.292-.382 2.693 2.693 0 0 0-.372-.343 1.841 1.841 0 0 0-.432-.24 1.2 1.2 0 0 0-.481-.101c-.04.001-.08.005-.12.01a.649.649 0 0 0-.162.02.408.408 0 0 0-.13.06.116.116 0 0 0-.06.1.33.33 0 0 0 .14.242c.093.074.17.131.232.171l.03.021c.133.103.261.214.382.333.112.098.213.209.3.33.09.119.168.246.231.381.073.134.15.288.231.463.188.474.522.875.954 1.145.453.243.961.364 1.476.351.174 0 .349-.01.522-.03.172-.028.343-.057.515-.091v1.743a.5.5 0 0 1-.533.521h-.062a10.286 10.286 0 1 1 6.324 0v.005z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGlobe;
impl IconShape for VsGlobe {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.5 1a6.5 6.5 0 1 1 0 13 6.5 6.5 0 0 1 0-13zm4.894 4a5.527 5.527 0 0 0-3.053-2.676c.444.84.765 1.74.953 2.676h2.1zm.582 2.995A5.11 5.11 0 0 0 14 7.5a5.464 5.464 0 0 0-.213-1.5h-2.342c.032.331.055.664.055 1a10.114 10.114 0 0 1-.206 2h2.493c.095-.329.158-.665.19-1.005zm-3.535 0l.006-.051A9.04 9.04 0 0 0 10.5 7a8.994 8.994 0 0 0-.076-1H6.576A8.82 8.82 0 0 0 6.5 7a8.98 8.98 0 0 0 .233 2h3.534c.077-.332.135-.667.174-1.005zM10.249 5a8.974 8.974 0 0 0-1.255-2.97C8.83 2.016 8.666 2 8.5 2a3.62 3.62 0 0 0-.312.015l-.182.015L8 2.04A8.97 8.97 0 0 0 6.751 5h3.498zM5.706 5a9.959 9.959 0 0 1 .966-2.681A5.527 5.527 0 0 0 3.606 5h2.1zM3.213 6A5.48 5.48 0 0 0 3 7.5 5.48 5.48 0 0 0 3.213 9h2.493A10.016 10.016 0 0 1 5.5 7c0-.336.023-.669.055-1H3.213zm2.754 4h-2.36a5.515 5.515 0 0 0 3.819 2.893A10.023 10.023 0 0 1 5.967 10zM8.5 12.644A8.942 8.942 0 0 0 9.978 10H7.022A8.943 8.943 0 0 0 8.5 12.644zM11.033 10a10.024 10.024 0 0 1-1.459 2.893A5.517 5.517 0 0 0 13.393 10h-2.36z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGoToEditingSession;
impl IconShape for VsGoToEditingSession {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 1.00006H3L2 2.00006V14.0001L3 15.0001H12L13 14.0001V6.81723H12V14.0001H3V2.00006H8V1.00006Z",
            }
            path {
                d: "M8 4.00006H7.00006V6.00006H5.00006V7.00006H7.00006V9.00006H8.00006V7.00006H10.0001V6.00008L8.00006 6.00006L8 4.00006Z",
            }
            path {
                d: "M5.00006 11.0001H10.0001V12.0001H5.00006V11.0001Z",
            }
            path {
                d: "M13.8703 0.482666L14.3724 0.984712V4.52025H13.3683V2.18679L10.2287 5.32635L9.5216 4.61924L12.6682 1.47262L10.3348 1.47262L10.3348 0.482666L13.8703 0.482666Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGoToFile;
impl IconShape for VsGoToFile {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 5.914l2.06-2.06v-.708L5.915 1l-.707.707.043.043.25.25 1 1h-3a2.5 2.5 0 0 0 0 5H4V7h-.5a1.5 1.5 0 1 1 0-3h3L5.207 5.293 5.914 6 6 5.914zM11 2H8.328l-1-1H12l.71.29 3 3L16 5v9l-1 1H6l-1-1V6.5l1 .847V14h9V6h-4V2zm1 0v3h3l-3-3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGoToSearch;
impl IconShape for VsGoToSearch {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.80005 10.15L1 13.84L1.75 14.5L5.53003 10.83L4.80005 10.15Z",
            }
            path {
                d: "M5.91418 6L8.06063 3.85356L8.06063 3.14645L5.91419 1L5.20708 1.70711L6.49997 3H3C1.61929 3 0.5 4.11929 0.5 5.5C0.5 6.76271 1.43615 7.80678 2.65227 7.97601C2.58806 7.66959 2.53498 7.31409 2.51211 6.91887C1.9232 6.71641 1.5 6.15763 1.5 5.5C1.5 4.67157 2.17157 4 3 4L6.49997 4L5.20708 5.29289L5.91418 6Z",
            }
            path {
                d: "M8.23663 2.02947L7.41223 1.20507C7.70346 1.12323 7.99962 1.06592 8.2977 1.03308C9.1038 0.944267 9.9239 1.03438 10.6981 1.30221C11.791 1.68027 12.7369 2.39318 13.4014 3.33962C14.0659 4.28606 14.4152 5.41787 14.3996 6.57418C14.384 7.73049 14.0043 8.85247 13.3145 9.78064C12.6248 10.7088 11.66 11.396 10.5573 11.7444C9.45461 12.0928 8.27017 12.0848 7.17229 11.7216C6.07442 11.3583 5.11898 10.6582 4.44178 9.7208C3.96202 9.05671 3.63921 8.29746 3.49175 7.5C3.4612 7.3348 3.43818 7.16797 3.42285 7H4.40833C4.49429 7.7722 4.7783 8.51224 5.23659 9.14662C5.79306 9.91689 6.57816 10.4922 7.48031 10.7907C8.38246 11.0892 9.35576 11.0958 10.2619 10.8095C11.1679 10.5231 11.9607 9.95849 12.5275 9.19579C13.0944 8.43308 13.4063 7.51113 13.4192 6.56096C13.432 5.61079 13.145 4.68075 12.5989 3.90304C12.0529 3.12532 11.2756 2.53951 10.3776 2.22885C9.68735 1.99008 8.95277 1.92319 8.23663 2.02947Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGrabber;
impl IconShape for VsGrabber {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M15 6H1v1h14V6zm0 3H1v1h14V9z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGraphLeft;
impl IconShape for VsGraphLeft {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.119 4L3 4.881l-.619.619L.715 3.833v-.618L2.38 1.548l.62.619L2.167 3H15v1H2.119zM4 14.546V5.455L4.5 5h2l.5.455v9.09L6.5 15h-2l-.5-.454zm2-.455V5.909H5v8.182h1zm2-1.535V5.444L8.5 5h2l.5.444v7.112l-.5.444h-2l-.5-.444zm2-.445V5.89H9v6.222h1zm2-6.682v5.143l.5.428h2l.5-.428V5.429L14.5 5h-2l-.5.429zm2 .428v4.286h-1V5.857h1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGraphLine;
impl IconShape for VsGraphLine {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 13V14H1.5L1 13.5V0H2V13H15Z",
            }
            path {
                d: "M13 3.20714L7.85353 8.35359H7.14642L5.49998 6.70714L1.85353 10.3536L1.14642 9.64648L5.14642 5.64648H5.85353L7.49998 7.29293L12.6464 2.14648H13.3535L15.3535 4.14648L14.6464 4.85359L13 3.20714Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGraphScatter;
impl IconShape for VsGraphScatter {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 13V14H1.5L1 13.5V0H2V13H15Z",
            }
            rect {
                height: "2",
                width: "2",
                x: "5",
                y: "2",
            }
            rect {
                height: "2",
                width: "2",
                x: "12",
                y: "1",
            }
            rect {
                height: "2",
                width: "2",
                x: "8",
                y: "5",
            }
            rect {
                height: "2",
                width: "2",
                x: "5",
                y: "9",
            }
            rect {
                height: "2",
                width: "2",
                x: "12",
                y: "8",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGraph;
impl IconShape for VsGraph {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 14H15v-1H2V0H1v13.5l.5.5zM3 11.5v-8l.5-.5h2l.5.5v8l-.5.5h-2l-.5-.5zm2-.5V4H4v7h1zm6-9.5v10l.5.5h2l.5-.5v-10l-.5-.5h-2l-.5.5zm2 .5v9h-1V2h1zm-6 9.5v-6l.5-.5h2l.5.5v6l-.5.5h-2l-.5-.5zm2-.5V6H8v5h1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGripper;
impl IconShape for VsGripper {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 3h2v2H5zm0 4h2v2H5zm0 4h2v2H5zm4-8h2v2H9zm0 4h2v2H9zm0 4h2v2H9z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsGroupByRefType;
impl IconShape for VsGroupByRefType {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 1h2v1H2v12h1.5v1h-2l-.5-.5v-13l.5-.5zm6 6h-2L5 6.5v-2l.5-.5h2l.5.5v2l-.5.5zM6 6h1V5H6v1zm7.5 1h-3l-.5-.5v-3l.5-.5h3l.5.5v3l-.5.5zM11 6h2V4h-2v2zm-3.5 6h-2l-.5-.5v-2l.5-.5h2l.5.5v2l-.5.5zM6 11h1v-1H6v1zm7.5 2h-3l-.5-.5v-3l.5-.5h3l.5.5v3l-.5.5zM11 12h2v-2h-2v2zm-1-2H8v1h2v-1zm0-5H8v1h2V5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsHeartFilled;
impl IconShape for VsHeartFilled {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.88 4.78079C14.7993 4.46498 14.6748 4.16202 14.51 3.88077C14.3518 3.58819 14.1493 3.3217 13.91 3.09073C13.563 2.74486 13.152 2.46982 12.7 2.28079C11.7902 1.90738 10.7698 1.90738 9.85999 2.28079C9.43276 2.46163 9.04027 2.71541 8.70002 3.03079L8.65003 3.09073L8.00001 3.74075L7.34999 3.09073L7.3 3.03079C6.95975 2.71541 6.56726 2.46163 6.14002 2.28079C5.23018 1.90738 4.20984 1.90738 3.3 2.28079C2.84798 2.46982 2.43706 2.74486 2.09004 3.09073C1.85051 3.32402 1.64514 3.59002 1.48002 3.88077C1.32258 4.1644 1.20161 4.46682 1.12 4.78079C1.03522 5.10721 0.994861 5.44358 1.00001 5.78079C1.00053 6.09791 1.04084 6.41365 1.12 6.72073C1.20384 7.03078 1.32472 7.32961 1.48002 7.61075C1.64774 7.89975 1.85285 8.16542 2.09004 8.40079L8.00001 14.3108L13.91 8.40079C14.1471 8.16782 14.3492 7.90169 14.51 7.61075C14.6729 7.33211 14.7974 7.03272 14.88 6.72073C14.9592 6.41365 14.9995 6.09791 15 5.78079C15.0052 5.44358 14.9648 5.10721 14.88 4.78079Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsHeart;
impl IconShape for VsHeart {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.88 4.78a3.489 3.489 0 0 0-.37-.9 3.24 3.24 0 0 0-.6-.79 3.78 3.78 0 0 0-1.21-.81 3.74 3.74 0 0 0-2.84 0 4 4 0 0 0-1.16.75l-.05.06-.65.65-.65-.65-.05-.06a4 4 0 0 0-1.16-.75 3.74 3.74 0 0 0-2.84 0 3.78 3.78 0 0 0-1.21.81 3.55 3.55 0 0 0-.97 1.69 3.75 3.75 0 0 0-.12 1c0 .317.04.633.12.94a4 4 0 0 0 .36.89 3.8 3.8 0 0 0 .61.79L8 14.31l5.91-5.91c.237-.233.44-.5.6-.79A3.578 3.578 0 0 0 15 5.78a3.747 3.747 0 0 0-.12-1zm-1 1.63a2.69 2.69 0 0 1-.69 1.21l-5.21 5.2-5.21-5.2a2.9 2.9 0 0 1-.44-.57 3 3 0 0 1-.27-.65 3.25 3.25 0 0 1-.08-.69A3.36 3.36 0 0 1 2.06 5a2.8 2.8 0 0 1 .27-.65c.12-.21.268-.4.44-.57a2.91 2.91 0 0 1 .89-.6 2.8 2.8 0 0 1 2.08 0c.33.137.628.338.88.59l1.36 1.37 1.36-1.37a2.72 2.72 0 0 1 .88-.59 2.8 2.8 0 0 1 2.08 0c.331.143.633.347.89.6.174.165.32.357.43.57a2.69 2.69 0 0 1 .35 1.34 2.6 2.6 0 0 1-.06.72h-.03z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsHistory;
impl IconShape for VsHistory {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13.507 12.324a7 7 0 0 0 .065-8.56A7 7 0 0 0 2 4.393V2H1v3.5l.5.5H5V5H2.811a6.008 6.008 0 1 1-.135 5.77l-.887.462a7 7 0 0 0 11.718 1.092zm-3.361-.97l.708-.707L8 7.792V4H7v4l.146.354 3 3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsHome;
impl IconShape for VsHome {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.36 1.37l6.36 5.8-.71.71L13 6.964v6.526l-.5.5h-3l-.5-.5v-3.5H7v3.5l-.5.5h-3l-.5-.5V6.972L2 7.88l-.71-.71 6.35-5.8h.72zM4 6.063v6.927h2v-3.5l.5-.5h3l.5.5v3.5h2V6.057L8 2.43 4 6.063z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsHorizontalRule;
impl IconShape for VsHorizontalRule {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6.432 10h.823V4h-.823v2.61h-2.61V4H3v6h.823V7.394h2.61V10zm5.668 0h.9l-1.28-2.63c.131-.058.26-.134.389-.23a1.666 1.666 0 0 0 .585-.797c.064-.171.096-.364.096-.58a1.77 1.77 0 0 0-.082-.557 1.644 1.644 0 0 0-.22-.446 1.504 1.504 0 0 0-.31-.341 1.864 1.864 0 0 0-.737-.373A1.446 1.446 0 0 0 11.1 4H8.64v6h.824V7.518h1.467L12.1 10zm-.681-3.32a.874.874 0 0 1-.293.055H9.463V4.787h1.663a.87.87 0 0 1 .576.24.956.956 0 0 1 .306.737c0 .168-.029.314-.087.437a.91.91 0 0 1-.503.479zM13 12H3v1h10v-1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsHubot;
impl IconShape for VsHubot {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.48 4h4l.5.5v2.03h.52l.5.5V8l-.5.5h-.52v3l-.5.5H9.36l-2.5 2.76L6 14.4V12H3.5l-.5-.64V8.5h-.5L2 8v-.97l.5-.5H3V4.36L3.53 4h4V2.86A1 1 0 0 1 7 2a1 1 0 0 1 2 0 1 1 0 0 1-.52.83V4zM12 8V5H4v5.86l2.5.14H7v2.19l1.8-2.04.35-.15H12V8zm-2.12.51a2.71 2.71 0 0 1-1.37.74v-.01a2.71 2.71 0 0 1-2.42-.74l-.7.71c.34.34.745.608 1.19.79.45.188.932.286 1.42.29a3.7 3.7 0 0 0 2.58-1.07l-.7-.71zM6.49 6.5h-1v1h1v-1zm3 0h1v1h-1v-1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsInbox;
impl IconShape for VsInbox {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 14h13l.5-.5V9l-2.77-7.66-.47-.34H4.27l-.47.33L1 8.74v4.76l.5.5zM14 13H2v-2.98h2.55l.74 1.25.43.24h4.57l.44-.26.69-1.23H14V13zm-.022-3.98H11.12l-.43.26-.69 1.23H6.01l-.75-1.25-.43-.24H2V9l2.62-7h6.78l2.578 7.02z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsIndent;
impl IconShape for VsIndent {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 5V6.984C4 7.11661 4.0527 7.24379 4.1465 7.33755C4.2402 7.43132 4.3674 7.484 4.5 7.484H11.382L9.749 5.851L10.456 5.144L12.577 7.265L13 7.688V8.256L10.456 10.8L9.749 10.093L11.359 8.484H4.5C4.1022 8.484 3.7207 8.32597 3.4393 8.04466C3.158 7.76336 3 7.38182 3 6.984V5H4Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsInfo;
impl IconShape for VsInfo {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.568 1.031A6.8 6.8 0 0 1 12.76 3.05a7.06 7.06 0 0 1 .46 9.39 6.85 6.85 0 0 1-8.58 1.74 7 7 0 0 1-3.12-3.5 7.12 7.12 0 0 1-.23-4.71 7 7 0 0 1 2.77-3.79 6.8 6.8 0 0 1 4.508-1.149zM9.04 13.88a5.89 5.89 0 0 0 3.41-2.07 6.07 6.07 0 0 0-.4-8.06 5.82 5.82 0 0 0-7.43-.74 6.06 6.06 0 0 0 .5 10.29 5.81 5.81 0 0 0 3.92.58zM7.375 6h1.25V5h-1.25v1zm1.25 1v4h-1.25V7h1.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsInsert;
impl IconShape for VsInsert {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14 1L15 2V6L14 7L6 7L5 6L5 2L6 1L14 1ZM14 2L6 2L6 6L14 6L14 2Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M14 9L15 10V14L14 15L6 15L5 14L5 10L6 9L14 9ZM14 10L6 10L6 14L14 14L14 10Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M1 6.39268L2.61414 8.00682L1 9.62096L1.69352 10.3141L4 8.00682L1.69352 5.69995L1 6.39268Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsInspect;
impl IconShape for VsInspect {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1 3l1-1h12l1 1v6h-1V3H2v8h5v1H2l-1-1V3zm14.707 9.707L9 6v9.414l2.707-2.707h4zM10 13V8.414l3.293 3.293h-2L10 13z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsIssueDraft;
impl IconShape for VsIssueDraft {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.7324 9.20047L13.6835 9.50931C13.889 8.87656 14 8.20125 14 7.5C14 6.79875 13.889 6.12344 13.6835 5.49069L12.7324 5.79953C12.9058 6.33376 13 6.9049 13 7.5C13 8.0951 12.9058 8.66624 12.7324 9.20047ZM12.4021 5.00313L13.2928 4.54842C12.6696 3.3279 11.6721 2.33037 10.4516 1.70723L9.99687 2.59787C11.0298 3.12523 11.8748 3.9702 12.4021 5.00313ZM9.20047 2.26763L9.50931 1.31652C8.87656 1.11105 8.20125 1 7.5 1C6.79875 1 6.12344 1.11105 5.49069 1.31652L5.79953 2.26763C6.33376 2.09415 6.9049 2 7.5 2C8.0951 2 8.66624 2.09415 9.20047 2.26763ZM5.00313 2.59787L4.54842 1.70723C3.3279 2.33037 2.33037 3.3279 1.70723 4.54842L2.59787 5.00313C3.12523 3.9702 3.9702 3.12523 5.00313 2.59787ZM1 7.5C1 6.79875 1.11105 6.12344 1.31652 5.49069L2.26763 5.79953C2.09415 6.33376 2 6.9049 2 7.5C2 8.0951 2.09415 8.66624 2.26763 9.20047L1.31652 9.50931C1.11105 8.87656 1 8.20125 1 7.5ZM2.59787 9.99687L1.70723 10.4516C2.33037 11.6721 3.3279 12.6696 4.54842 13.2928L5.00313 12.4021C3.9702 11.8748 3.12523 11.0298 2.59787 9.99687ZM5.79953 12.7324L5.49069 13.6835C6.12344 13.889 6.79875 14 7.5 14C8.20125 14 8.87656 13.889 9.50931 13.6835L9.20047 12.7324C8.66624 12.9058 8.0951 13 7.5 13C6.9049 13 6.33376 12.9058 5.79953 12.7324ZM9.99687 12.4021L10.4516 13.2928C11.6721 12.6696 12.6696 11.6721 13.2928 10.4516L12.4021 9.99687C11.8748 11.0298 11.0298 11.8748 9.99687 12.4021ZM7.50002 8.5C8.0523 8.5 8.50002 8.05228 8.50002 7.5C8.50002 6.94772 8.0523 6.5 7.50002 6.5C6.94773 6.5 6.50002 6.94772 6.50002 7.5C6.50002 8.05228 6.94773 8.5 7.50002 8.5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsIssueReopened;
impl IconShape for VsIssueReopened {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.28 5.65556L2 7.00556L1.34 6.74556L0 3.50556L0.92 3.12556L1.73 5.07556C2.27376 3.71475 3.2627 2.57844 4.53544 1.85207C5.80817 1.1257 7.28953 0.852174 8.73774 1.07614C10.1859 1.3001 11.5155 2.00832 12.5093 3.08521C13.5032 4.1621 14.1027 5.54407 14.21 7.00556H13.21C13.0956 5.75683 12.5564 4.58511 11.6824 3.68594C10.8083 2.78677 9.65237 2.21456 8.40739 2.06478C7.1624 1.91501 5.90371 2.19674 4.84137 2.86297C3.77903 3.52919 2.97731 4.53959 2.57 5.72556L4.89 4.72556L5.28 5.65556ZM14.14 8.33562L15.48 11.5656L14.56 12.0056L13.74 10.0056C13.1919 11.3718 12.1958 12.511 10.9149 13.2364C9.63412 13.9618 8.14476 14.2302 6.69127 13.9977C5.23779 13.7651 3.90654 13.0454 2.91599 11.9566C1.92544 10.8678 1.33445 9.47455 1.24001 8.00562H2.24001V7.50562C2.24281 8.79308 2.69801 10.0386 3.52602 11.0245C4.35404 12.0104 5.5022 12.6739 6.76983 12.899C8.03745 13.1242 9.34388 12.8967 10.4608 12.2563C11.5777 11.6159 12.434 10.6033 12.88 9.39562L10.63 10.3256L10.24 9.40561L13.49 8.05562L14.14 8.33562Z",
                fill_rule: "evenodd",
            }
            circle {
                cx: "7.74001",
                cy: "7.53955",
                r: "1",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsIssues;
impl IconShape for VsIssues {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.50002 1C6.21445 1 4.95774 1.38123 3.88882 2.09546C2.8199 2.80969 1.98674 3.82485 1.49478 5.01257C1.00281 6.20029 0.874098 7.50719 1.1249 8.76807C1.37571 10.0289 1.99479 11.1872 2.90383 12.0962C3.81287 13.0052 4.97108 13.6243 6.23196 13.8751C7.49283 14.1259 8.79973 13.9972 9.98745 13.5052C11.1752 13.0133 12.1903 12.1801 12.9046 11.1112C13.6188 10.0423 14 8.78558 14 7.5C14 5.77609 13.3152 4.1228 12.0962 2.90381C10.8772 1.68482 9.22393 1 7.50002 1ZM7.50002 13C6.41223 13 5.34883 12.6775 4.44436 12.0731C3.53989 11.4688 2.83501 10.6097 2.41873 9.60474C2.00244 8.59974 1.89352 7.4939 2.10574 6.427C2.31796 5.36011 2.8418 4.38015 3.61099 3.61096C4.38018 2.84177 5.36013 2.31793 6.42703 2.10571C7.49392 1.89349 8.59977 2.00242 9.60476 2.4187C10.6098 2.83498 11.4688 3.53987 12.0731 4.44434C12.6775 5.34881 13 6.4122 13 7.5C13 8.95869 12.4205 10.3576 11.3891 11.389C10.3576 12.4205 8.95871 13 7.50002 13Z",
            }
            circle {
                cx: "7.50002",
                cy: "7.5",
                r: "1",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsItalic;
impl IconShape for VsItalic {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.001 13.593l-.097.325H4l.123-.325c.492-.012.817-.053.976-.123.257-.1.448-.238.57-.413.194-.276.394-.768.599-1.477l2.074-7.19c.176-.597.263-1.048.263-1.353a.643.643 0 0 0-.114-.387.683.683 0 0 0-.351-.237c-.153-.059-.454-.088-.906-.088L7.34 2h4.605l-.096.325c-.375-.006-.654.035-.835.123a1.358 1.358 0 0 0-.607.501c-.134.217-.31.697-.527 1.442l-2.066 7.19c-.187.661-.28 1.083-.28 1.265 0 .146.034.272.105.378.076.1.193.178.351.237.164.053.501.097 1.011.132z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsJersey;
impl IconShape for VsJersey {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.91 14.22H4.06l-.5-.5V7.06H2.15l-.48-.38L1 4l.33-.6L5.59 2l.64.32a2.7 2.7 0 0 0 .21.44c.071.103.152.2.24.29.168.169.369.302.59.39a1.82 1.82 0 0 0 1.43 0 1.74 1.74 0 0 0 .59-.39c.09-.095.173-.195.25-.3l.15-.29a1.21 1.21 0 0 0 .05-.14l.64-.32 4.26 1.42L15 4l-.66 2.66-.49.38h-1.44v6.66l-.5.52zm-7.35-1h6.85V6.56l.5-.5h1.52l.46-1.83-3.4-1.14a1.132 1.132 0 0 1-.12.21c-.11.161-.233.312-.37.45a2.75 2.75 0 0 1-.91.61 2.85 2.85 0 0 1-2.22 0A2.92 2.92 0 0 1 6 3.75a2.17 2.17 0 0 1-.36-.44l-.13-.22-3.43 1.14.46 1.83h1.52l.5.5v6.66z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsJson;
impl IconShape for VsJson {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2.984V2h-.09c-.313 0-.616.062-.909.185a2.33 2.33 0 0 0-.775.53 2.23 2.23 0 0 0-.493.753v.001a3.542 3.542 0 0 0-.198.83v.002a6.08 6.08 0 0 0-.024.863c.012.29.018.58.018.869 0 .203-.04.393-.117.572v.001a1.504 1.504 0 0 1-.765.787 1.376 1.376 0 0 1-.558.115H2v.984h.09c.195 0 .38.04.556.121l.001.001c.178.078.329.184.455.318l.002.002c.13.13.233.285.307.465l.001.002c.078.18.117.368.117.566 0 .29-.006.58-.018.869-.012.296-.004.585.024.87v.001c.033.283.099.558.197.824v.001c.106.273.271.524.494.753.223.23.482.407.775.53.293.123.596.185.91.185H6v-.984h-.09c-.2 0-.387-.038-.563-.115a1.613 1.613 0 0 1-.457-.32 1.659 1.659 0 0 1-.309-.467c-.074-.18-.11-.37-.11-.573 0-.228.003-.453.011-.672.008-.228.008-.45 0-.665a4.639 4.639 0 0 0-.055-.64 2.682 2.682 0 0 0-.168-.609A2.284 2.284 0 0 0 3.522 8a2.284 2.284 0 0 0 .738-.955c.08-.192.135-.393.168-.602.033-.21.051-.423.055-.64.008-.22.008-.442 0-.666-.008-.224-.012-.45-.012-.678a1.47 1.47 0 0 1 .877-1.354 1.33 1.33 0 0 1 .563-.121H6zm4 10.032V14h.09c.313 0 .616-.062.909-.185.293-.123.552-.3.775-.53.223-.23.388-.48.493-.753v-.001c.1-.266.165-.543.198-.83v-.002c.028-.28.036-.567.024-.863-.012-.29-.018-.58-.018-.869 0-.203.04-.393.117-.572v-.001a1.502 1.502 0 0 1 .765-.787 1.38 1.38 0 0 1 .558-.115H14v-.984h-.09c-.196 0-.381-.04-.557-.121l-.001-.001a1.376 1.376 0 0 1-.455-.318l-.002-.002a1.415 1.415 0 0 1-.307-.465v-.002a1.405 1.405 0 0 1-.118-.566c0-.29.006-.58.018-.869a6.174 6.174 0 0 0-.024-.87v-.001a3.537 3.537 0 0 0-.197-.824v-.001a2.23 2.23 0 0 0-.494-.753 2.331 2.331 0 0 0-.775-.53 2.325 2.325 0 0 0-.91-.185H10v.984h.09c.2 0 .387.038.562.115.174.082.326.188.457.32.127.134.23.29.309.467.074.18.11.37.11.573 0 .228-.003.452-.011.672-.008.228-.008.45 0 .665.004.222.022.435.055.64.033.214.089.416.168.609a2.285 2.285 0 0 0 .738.955 2.285 2.285 0 0 0-.738.955 2.689 2.689 0 0 0-.168.602c-.033.21-.051.423-.055.64a9.15 9.15 0 0 0 0 .666c.008.224.012.45.012.678a1.471 1.471 0 0 1-.877 1.354 1.33 1.33 0 0 1-.563.121H10z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsKebabVertical;
impl IconShape for VsKebabVertical {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.444 13.832a1 1 0 1 0 1.111-1.663 1 1 0 0 0-1.11 1.662zM8 9a1 1 0 1 1 0-2 1 1 0 0 1 0 2zm0-5a1 1 0 1 1 0-2 1 1 0 0 1 0 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsKey;
impl IconShape for VsKey {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.351 1.091a4.528 4.528 0 0 1 3.44 3.16c.215.724.247 1.49.093 2.23a4.583 4.583 0 0 1-4.437 3.6c-.438 0-.874-.063-1.293-.19l-.8.938-.379.175H7v1.5l-.5.5H5v1.5l-.5.5h-3l-.5-.5v-2.307l.146-.353L6.12 6.87a4.464 4.464 0 0 1-.2-1.405 4.528 4.528 0 0 1 5.431-4.375zm1.318 7.2a3.568 3.568 0 0 0 1.239-2.005l.004.005A3.543 3.543 0 0 0 9.72 2.08a3.576 3.576 0 0 0-2.8 3.4c-.01.456.07.908.239 1.33l-.11.543L2 12.404v1.6h2v-1.5l.5-.5H6v-1.5l.5-.5h1.245l.876-1.016.561-.14a3.47 3.47 0 0 0 1.269.238 3.568 3.568 0 0 0 2.218-.795zm-.838-2.732a1 1 0 1 0-1.662-1.11 1 1 0 0 0 1.662 1.11z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsKeyboardTab;
impl IconShape for VsKeyboardTab {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.00001 13.8871L14 8.88711V8.18L9.00001 3.18L8.2929 3.88711L12.4393 8.03355L2 8.03355L2 9.03355L12.4393 9.03355L8.2929 13.18L9.00001 13.8871Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M15 3H16V14H15V3Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLaw;
impl IconShape for VsLaw {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.63 7L13 3h1V2H9V1H8v1H3v1h1L2.38 7H2v1h.15c.156.498.473.93.9 1.23a2.47 2.47 0 0 0 2.9 0A2.44 2.44 0 0 0 6.86 8H7V7h-.45L4.88 3H8v8H6l-.39.18-2 2.51.39.81h9l.39-.81-2-2.51L11 11H9V3h3.13l-1.67 4H10v1h.15a2.48 2.48 0 0 0 4.71 0H15V7h-.37zM5.22 8.51a1.52 1.52 0 0 1-.72.19 1.45 1.45 0 0 1-.71-.19A1.47 1.47 0 0 1 3.25 8h2.5a1.52 1.52 0 0 1-.53.51zM5.47 7h-2l1-2.4 1 2.4zm5.29 5L12 13.5H5L6.24 12h4.52zm1.78-7.38l1 2.4h-2l1-2.4zm.68 3.91a1.41 1.41 0 0 1-.72.19 1.35 1.35 0 0 1-.71-.19 1.55 1.55 0 0 1-.54-.53h2.5a1.37 1.37 0 0 1-.53.53z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayersActive;
impl IconShape for VsLayersActive {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.18535 1.08325L7.62706 1.08717L1.71796 5.12422L1.72152 5.95233L7.63062 9.91528L8.1818 9.91912L14.2727 5.95617L14.2762 5.1203L8.18535 1.08325ZM2.89198 5.53323L7.91335 2.10268L13.0891 5.5332L7.91329 8.90079L2.89198 5.53323ZM7.63059 12.4153L1.79257 8.5H3.58794L7.91326 11.4008L12.3716 8.5H14.2053L13.4056 9.02031C13.2722 9.00688 13.1369 9 13 9C11.224 9 9.71839 10.1574 9.19622 11.7591L8.18177 12.4191L7.63059 12.4153ZM9.00447 13.1908L7.91326 13.9008L3.58794 11H1.79257L7.63059 14.9153L8.18177 14.9191L9.20113 14.2559C9.08965 13.9185 9.02187 13.5612 9.00447 13.1908Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M11.3333 10.5056C11.8266 10.1759 12.4067 10 13 10C13.7954 10.001 14.5578 10.3174 15.1202 10.8798C15.6826 11.4422 15.999 12.2046 16 13C16 13.5933 15.8241 14.1734 15.4944 14.6667C15.1648 15.1601 14.6962 15.5446 14.1481 15.7716C13.5999 15.9987 12.9967 16.0581 12.4147 15.9424C11.8328 15.8266 11.2982 15.5409 10.8787 15.1213C10.4591 14.7018 10.1734 14.1672 10.0576 13.5853C9.94189 13.0033 10.0013 12.4001 10.2284 11.8519C10.4554 11.3038 10.8399 10.8352 11.3333 10.5056ZM13.0315 14.3226L14.8213 11.9363L14.0213 11.3363L12.541 13.3099L11.6655 12.6095L11.0408 13.3903L12.3192 14.413L13.0315 14.3226Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayersDot;
impl IconShape for VsLayersDot {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.18535 1.08325L7.62706 1.08717L1.71796 5.12422L1.72152 5.95233L7.63062 9.91528L8.1818 9.91912L14.2727 5.95617L14.2762 5.1203L8.18535 1.08325ZM2.89198 5.53323L7.91335 2.10268L13.0891 5.5332L7.91329 8.90079L2.89198 5.53323ZM7.63059 12.4153L1.79257 8.5H3.58794L7.91326 11.4008L12.3716 8.5H14.2053L13.4056 9.02031C13.2722 9.00688 13.1369 9 13 9C11.224 9 9.71839 10.1574 9.19622 11.7591L8.18177 12.4191L7.63059 12.4153ZM9.00447 13.1908L7.91326 13.9008L3.58794 11H1.79257L7.63059 14.9153L8.18177 14.9191L9.20113 14.2559C9.08965 13.9185 9.02187 13.5612 9.00447 13.1908Z",
                fill_rule: "evenodd",
            }
            circle {
                cx: "13",
                cy: "13",
                r: "3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayers;
impl IconShape for VsLayers {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.62706 1.08717L8.18535 1.08325L14.2762 5.1203L14.2727 5.95617L8.1818 9.91912L7.63062 9.91528L1.72152 5.95233L1.71796 5.12422L7.62706 1.08717ZM7.91335 2.10268L2.89198 5.53323L7.91329 8.90079L13.0891 5.5332L7.91335 2.10268ZM1.79257 8.5L7.63059 12.4153L8.18177 12.4191L14.2053 8.5H12.3716L7.91326 11.4008L3.58794 8.5H1.79257ZM7.63059 14.9153L1.79257 11H3.58794L7.91326 13.9008L12.3716 11H14.2053L8.18177 14.9191L7.63059 14.9153Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayoutActivitybarLeft;
impl IconShape for VsLayoutActivitybarLeft {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 1L1 2V14L2 15H14L15 14V2L14 1H2ZM14 14H4V2H14V14Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayoutActivitybarRight;
impl IconShape for VsLayoutActivitybarRight {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 1L1 2V14L2 15H14L15 14V2L14 1H2ZM2 14V2H12V14H2Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayoutCentered;
impl IconShape for VsLayoutCentered {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 1L1 2V14L2 15H14L15 14V2L14 1H2ZM2 14V2H6V14H2ZM10 14V2H14V14H10Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayoutMenubar;
impl IconShape for VsLayoutMenubar {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1 2.00085L2 1.00085H14L15 2.00085V14.0009L14 15.0009H2L1 14.0009V2.00085ZM2 2.00085V14.0009H14V2.00085H2ZM3 3.00085H5V4.00085H3V3.00085ZM6 3.00085H8V4.00085H6V3.00085ZM11 3.00085H9V4.00085H11V3.00085Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayoutPanelCenter;
impl IconShape for VsLayoutPanelCenter {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 1L1 2V14L2 15H14L15 14V2L14 1H2ZM2 14V2H4V14H2ZM5 10V2H11V10H5ZM12 2H14V14H12V2Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayoutPanelJustify;
impl IconShape for VsLayoutPanelJustify {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 1L1 2V14L2 15H14L15 14V2L14 1H2ZM2 10V2H4V10H2ZM5 10V2H11V10H5ZM12 10V2H14V10H12Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayoutPanelLeft;
impl IconShape for VsLayoutPanelLeft {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1 2L2 1H14L15 2V14L14 15H2L1 14V2ZM2 2V10H10V2H2ZM11 2V14H14V2H11Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayoutPanelOff;
impl IconShape for VsLayoutPanelOff {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 1.00073L1 2.00073V14.0007L2 15.0007H14L15 14.0007V2.00073L14 1.00073H2ZM2 10.0007V2.00073H14V10.0007H2ZM2 11.0007H14V14.0007H2V11.0007Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayoutPanelRight;
impl IconShape for VsLayoutPanelRight {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1 2L2 1H14L15 2V14L14 15H2L1 14V2ZM2 2V14H5V2H2ZM6 2V10H14V2H6Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayoutPanel;
impl IconShape for VsLayoutPanel {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 1L1 2V14L2 15H14L15 14V2L14 1H2ZM2 10V2H14V10H2Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayoutSidebarLeftOff;
impl IconShape for VsLayoutSidebarLeftOff {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 1.00073L1 2.00073V14.0007L2 15.0007H14L15 14.0007V2.00073L14 1.00073H2ZM2 14.0007V2.00073H6V14.0007H2ZM7 14.0007V2.00073H14V14.0007H7Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayoutSidebarLeft;
impl IconShape for VsLayoutSidebarLeft {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 1L1 2V14L2 15H14L15 14V2L14 1H2ZM14 14H7V2H14V14Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayoutSidebarRightOff;
impl IconShape for VsLayoutSidebarRightOff {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 1.00073L1 2.00073V14.0007L2 15.0007H14L15 14.0007V2.00073L14 1.00073H2ZM2 14.0007V2.00073H9V14.0007H2ZM10 14.0007V2.00073H14V14.0007H10Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayoutSidebarRight;
impl IconShape for VsLayoutSidebarRight {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 1L1 2V14L2 15H14L15 14V2L14 1H2ZM2 14V2H9V14H2Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayoutStatusbar;
impl IconShape for VsLayoutStatusbar {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 1.00073L1 2.00073V14.0007L2 15.0007H14L15 14.0007V2.00073L14 1.00073H2ZM2 13.0007V2.00073H14V13.0007H2Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLayout;
impl IconShape for VsLayout {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 2L2 3V13L3 14H7L8 13V3L7 2H3ZM3 13V3H7V13H3Z",
            }
            path {
                d: "M10 3L11 2H14L15 3V6L14 7H11L10 6V3ZM11 3V6H14V3H11Z",
            }
            path {
                d: "M10 10L11 9H14L15 10V13L14 14H11L10 13V10ZM11 10V13H14V10H11Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLibrary;
impl IconShape for VsLibrary {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 2.5l.5-.5h2l.5.5v11l-.5.5h-2l-.5-.5v-11zM6 3v10h1V3H6zm3.171.345l.299-.641 1.88-.684.64.299 3.762 10.336-.299.641-1.879.684-.64-.299L9.17 3.345zm1.11.128l3.42 9.396.94-.341-3.42-9.397-.94.342zM1 2.5l.5-.5h2l.5.5v11l-.5.5h-2l-.5-.5v-11zM2 3v10h1V3H2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLightbulbAutofix;
impl IconShape for VsLightbulbAutofix {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 9a3 3 0 1 0 0 6 3 3 0 0 0 0-6zm1.31 5L12 13l-1.3 1 .5-1.53-1.2-.83h1.47L12 10l.54 1.64H14l-1.2.83.51 1.53z",
            }
            path {
                clip_rule: "evenodd",
                d: "M11.17 8.085A3.979 3.979 0 0 0 8.288 10.5H6.409v2.201c0 .081.028.15.09.212a.29.29 0 0 0 .213.09h1.413c.089.348.223.678.396.982-.066.01-.134.015-.203.015H6.712a1.285 1.285 0 0 1-.922-.379 1.303 1.303 0 0 1-.38-.92v-1.6c0-.479-.092-.921-.274-1.329a3.556 3.556 0 0 0-.776-1.114 4.689 4.689 0 0 1-1.006-1.437A4.187 4.187 0 0 1 3 5.5a4.432 4.432 0 0 1 .616-2.27c.197-.336.432-.64.705-.914a4.6 4.6 0 0 1 .911-.702c.338-.196.7-.348 1.084-.454a4.45 4.45 0 0 1 1.2-.16 4.476 4.476 0 0 1 2.276.614 4.475 4.475 0 0 1 1.622 1.616 4.438 4.438 0 0 1 .616 2.27c0 .617-.117 1.191-.353 1.721a4.537 4.537 0 0 1-.506.864z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLightbulbEmpty;
impl IconShape for VsLightbulbEmpty {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.5 6.5C4.5 4.567 6.067 3 8 3C9.933 3 11.5 4.567 11.5 6.5C11.5 7.47709 11.1003 8.35991 10.4542 8.99541C10.2612 9.18524 10.0912 9.43189 10.0095 9.73139L9.66356 11H6.33644L5.99046 9.73139C5.90878 9.43189 5.73883 9.18524 5.54584 8.99541C4.89973 8.35991 4.5 7.47709 4.5 6.5ZM6.60917 12H9.39083L9.21859 12.6316C9.15926 12.8491 8.96168 13 8.7362 13H7.2638C7.03832 13 6.84074 12.8491 6.78141 12.6316L6.60917 12ZM8 2C5.51472 2 3.5 4.01472 3.5 6.5C3.5 7.75601 4.01523 8.89258 4.8446 9.70834C4.94136 9.80352 5.00044 9.9019 5.02569 9.9945L5.81665 12.8947C5.99463 13.5473 6.58737 14 7.2638 14H8.7362C9.41263 14 10.0054 13.5473 10.1834 12.8947L10.9743 9.9945C10.9996 9.9019 11.0586 9.80352 11.1554 9.70834C11.9848 8.89258 12.5 7.75601 12.5 6.5C12.5 4.01472 10.4853 2 8 2Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLightbulbSparkle;
impl IconShape for VsLightbulbSparkle {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.96712 9.60787C9.14342 9.26307 9.37775 8.94649 9.67076 8.65806C9.85736 8.47395 10.0257 8.28064 10.1757 8.07816C10.2158 7.96017 10.2532 7.8333 10.2873 7.69683C10.3615 7.40021 10.5382 7.15079 10.7739 6.98253C10.945 6.51978 11.0303 6.0252 11.0303 5.49953C11.0303 5.08664 10.9769 4.68802 10.8698 4.30397C10.763 3.92088 10.6112 3.56285 10.4143 3.23016C10.2177 2.89376 9.98251 2.5891 9.70882 2.31641C9.43513 2.0437 9.12939 1.80938 8.79183 1.61351C8.45803 1.41739 8.09883 1.2662 7.71451 1.15983C7.32923 1.0532 6.92934 1 6.51514 1C6.10094 1 5.70106 1.0532 5.31578 1.15983C4.93146 1.2662 4.56979 1.41764 4.23195 1.61364C3.89858 1.80953 3.59503 2.04383 3.32146 2.31641C3.04777 2.58911 2.81257 2.89377 2.61595 3.23018C2.41907 3.56286 2.26728 3.92089 2.16048 4.30397C2.05342 4.68802 2 5.08664 2 5.49953C2 6.11672 2.11756 6.69107 2.35361 7.22134C2.58896 7.75003 2.92468 8.22903 3.35953 8.65806C3.69832 8.99156 3.95683 9.36336 4.13553 9.77209C4.31772 10.1795 4.40927 10.622 4.40927 11.1009V12.7012C4.40927 12.8807 4.44311 13.0503 4.51141 13.2091C4.57895 13.3661 4.67168 13.5038 4.78961 13.6213C4.90753 13.7388 5.04564 13.8311 5.20306 13.8984C5.36223 13.9663 5.53223 14 5.71205 14H7.31823C7.49806 14 7.66806 13.9663 7.82723 13.8984C7.98464 13.8311 8.12275 13.7388 8.24068 13.6213C8.35861 13.5038 8.45134 13.3661 8.51887 13.2091C8.58718 13.0503 8.62102 12.8807 8.62102 12.7012V12.2734C8.61586 12.2723 8.61079 12.2712 8.60581 12.2701C8.54613 12.2576 8.50299 12.2525 8.48241 12.2506L8.47329 12.2498C8.1415 12.2429 7.8415 12.1066 7.62162 11.8894V12.7012C7.62162 12.7823 7.59309 12.8512 7.5314 12.9127C7.46971 12.9741 7.40022 13.0028 7.31823 13.0028H5.71205C5.63007 13.0028 5.56058 12.9741 5.49888 12.9127C5.4372 12.8512 5.40867 12.7823 5.40867 12.7012V10.5H7.35405C7.54359 10.0663 7.97213 9.76091 8.4732 9.75037L8.48232 9.7496C8.50289 9.74769 8.54604 9.74259 8.60572 9.73002C8.69843 9.7105 8.82494 9.67428 8.96712 9.60787Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M11.5 14C12.25 11.0001 14.5 11.0001 14.5 11.0001C14.5 11.0001 12.25 11 11.5 8C10.75 11 8.5 11.0001 8.5 11.0001C8.5 11.0001 10.75 11 11.5 14Z",
                stroke: "#FFCC00",
                stroke_linejoin: "round",
                stroke_width: "0.75",
            }
            path {
                d: "M12.926 13.2393C13.2849 12.977 13.5538 12.6663 13.7328 12.4216C13.8545 12.2553 14.1455 12.2553 14.2672 12.4216C14.4462 12.6663 14.7151 12.977 15.074 13.2393C15.2403 13.3609 15.2403 13.6393 15.074 13.7609C14.7151 14.0231 14.4462 14.3337 14.2672 14.5784C14.1455 14.7447 13.8545 14.7447 13.7328 14.5784C13.5538 14.3337 13.2849 14.0231 12.926 13.7609C12.7597 13.6393 12.7597 13.3609 12.926 13.2393Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLightbulb;
impl IconShape for VsLightbulb {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.67 8.658a3.661 3.661 0 0 0-.781 1.114 3.28 3.28 0 0 0-.268 1.329v1.6a1.304 1.304 0 0 1-.794 1.197 1.282 1.282 0 0 1-.509.102H7.712a1.285 1.285 0 0 1-.922-.379 1.303 1.303 0 0 1-.38-.92v-1.6c0-.479-.092-.921-.274-1.329a3.556 3.556 0 0 0-.776-1.114 4.689 4.689 0 0 1-1.006-1.437A4.187 4.187 0 0 1 4 5.5a4.432 4.432 0 0 1 .616-2.27c.197-.336.432-.64.705-.914a4.6 4.6 0 0 1 .911-.702c.338-.196.7-.348 1.084-.454a4.45 4.45 0 0 1 1.2-.16 4.476 4.476 0 0 1 2.276.614 4.475 4.475 0 0 1 1.622 1.616 4.438 4.438 0 0 1 .616 2.27c0 .617-.117 1.191-.353 1.721a4.69 4.69 0 0 1-1.006 1.437zM9.623 10.5H7.409v2.201c0 .081.028.15.09.212a.29.29 0 0 0 .213.09h1.606a.289.289 0 0 0 .213-.09.286.286 0 0 0 .09-.212V10.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLinkExternal;
impl IconShape for VsLinkExternal {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.5 1H6v1H2v12h12v-4h1v4.5l-.5.5h-13l-.5-.5v-13l.5-.5z",
            }
            path {
                d: "M15 1.5V8h-1V2.707L7.243 9.465l-.707-.708L13.293 2H8V1h6.5l.5.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLink;
impl IconShape for VsLink {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4.4 3h3.085a3.4 3.4 0 0 1 3.4 3.4v.205A3.4 3.4 0 0 1 7.485 10H7V9h.485A2.4 2.4 0 0 0 9.88 6.61V6.4A2.4 2.4 0 0 0 7.49 4H4.4A2.4 2.4 0 0 0 2 6.4v.205A2.394 2.394 0 0 0 4 8.96v1a3.4 3.4 0 0 1-3-3.35V6.4A3.405 3.405 0 0 1 4.4 3zM12 7.04v-1a3.4 3.4 0 0 1 3 3.36v.205A3.405 3.405 0 0 1 11.605 13h-3.09A3.4 3.4 0 0 1 5.12 9.61V9.4A3.4 3.4 0 0 1 8.515 6H9v1h-.485A2.4 2.4 0 0 0 6.12 9.4v.205A2.4 2.4 0 0 0 8.515 12h3.09A2.4 2.4 0 0 0 14 9.61V9.4a2.394 2.394 0 0 0-2-2.36z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsListFilter;
impl IconShape for VsListFilter {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 12v-1h4v1H6zM4 7h8v1H4V7zm10-4v1H2V3h12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsListFlat;
impl IconShape for VsListFlat {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "1",
                width: "9",
                x: "2",
                y: "9",
            }
            rect {
                height: "1",
                width: "8",
                x: "2",
                y: "12",
            }
            rect {
                height: "1",
                width: "12",
                x: "2",
                y: "6",
            }
            rect {
                height: "1",
                width: "11",
                x: "2",
                y: "3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsListOrdered;
impl IconShape for VsListOrdered {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.287 2.326L2.692 2h.677v3h-.708V2.792l-.374.281v-.747zM5 3h10v1H5V3zm0 4h10v1H5V7zm10 4H5v1h10v-1zM3.742 7.626l.029-.039.065-.09a.84.84 0 0 0 .156-.507c0-.12-.02-.24-.057-.354a.848.848 0 0 0-.492-.529 1.123 1.123 0 0 0-.452-.082 1.094 1.094 0 0 0-.458.087.867.867 0 0 0-.479.522A1.038 1.038 0 0 0 2 6.957v.05h.81v-.05a.3.3 0 0 1 .046-.157.174.174 0 0 1 .057-.054.19.19 0 0 1 .172 0 .188.188 0 0 1 .056.06.24.24 0 0 1 .031.081.445.445 0 0 1-.036.29 1.309 1.309 0 0 1-.12.182l-1 1.138-.012.013v.54h1.988v-.7h-.9l.65-.724zm-.037 3.817c.046.032.086.07.12.114a.841.841 0 0 1 .167.55c0 .107-.017.213-.05.314a.792.792 0 0 1-.487.5 1.288 1.288 0 0 1-.48.079c-.115 0-.23-.016-.341-.049a.94.94 0 0 1-.258-.123.751.751 0 0 1-.182-.177 1.063 1.063 0 0 1-.116-.2A1.038 1.038 0 0 1 2 12.078v-.049h.814v.049c0 .027.003.055.009.082a.207.207 0 0 0 .03.074.14.14 0 0 0 .053.052.2.2 0 0 0 .157.008.159.159 0 0 0 .056-.039.22.22 0 0 0 .042-.075.417.417 0 0 0 .017-.126.483.483 0 0 0-.022-.163.2.2 0 0 0-.051-.08.138.138 0 0 0-.06-.029.537.537 0 0 0-.077-.007h-.161v-.645h.168a.241.241 0 0 0 .069-.011.164.164 0 0 0 .065-.034.175.175 0 0 0 .048-.067.286.286 0 0 0 .021-.121.28.28 0 0 0-.016-.1.166.166 0 0 0-.097-.099.2.2 0 0 0-.156.007.164.164 0 0 0-.055.053.344.344 0 0 0-.04.156v.049H2v-.049a.987.987 0 0 1 .18-.544.8.8 0 0 1 .179-.186.87.87 0 0 1 .262-.133c.114-.036.234-.053.354-.051.116-.001.231.01.344.036.092.021.18.055.263.1a.757.757 0 0 1 .32.318.73.73 0 0 1 .09.347.81.81 0 0 1-.167.528.562.562 0 0 1-.12.114z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsListSelection;
impl IconShape for VsListSelection {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1 12v-1h9v1H1zm0-5h14v1H1V7zm11-4v1H1V3h11z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsListTree;
impl IconShape for VsListTree {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "1",
                width: "9",
                x: "4",
                y: "9",
            }
            rect {
                height: "1",
                width: "7",
                x: "4",
                y: "12",
            }
            rect {
                height: "1",
                width: "10",
                x: "4",
                y: "6",
            }
            rect {
                height: "1",
                width: "11",
                x: "1",
                y: "3",
            }
            rect {
                height: "9",
                width: "1",
                x: "4",
                y: "4",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsListUnordered;
impl IconShape for VsListUnordered {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 3H1v1h1V3zm0 3H1v1h1V6zM1 9h1v1H1V9zm1 3H1v1h1v-1zm2-9h11v1H4V3zm11 3H4v1h11V6zM4 9h11v1H4V9zm11 3H4v1h11v-1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLiveShare;
impl IconShape for VsLiveShare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13.735 1.694L15.178 1l8.029 6.328v1.388l-8.029 6.072-1.443-.694v-2.776h-.59c-4.06-.02-6.71.104-10.61 5.163l-1.534-.493a8.23 8.23 0 0 1 .271-2.255 11.026 11.026 0 0 1 3.92-6.793 11.339 11.339 0 0 1 7.502-2.547h1.04v-2.7zm1.804 7.917v2.776l5.676-4.281-5.648-4.545v2.664h-2.86A9.299 9.299 0 0 0 5.77 8.848a10.444 10.444 0 0 0-2.401 4.122c3.351-3.213 6.19-3.359 9.798-3.359h2.373zm-7.647 5.896a4.31 4.31 0 1 1 4.788 7.166 4.31 4.31 0 0 1-4.788-7.166zm.955 5.728a2.588 2.588 0 1 0 2.878-4.302 2.588 2.588 0 0 0-2.878 4.302z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLoading;
impl IconShape for VsLoading {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13.917 7A6.002 6.002 0 0 0 2.083 7H1.071a7.002 7.002 0 0 1 13.858 0h-1.012z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLocation;
impl IconShape for VsLocation {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.832 2.688A4.056 4.056 0 0 0 8.02 1.5h-.04a4.056 4.056 0 0 0-4 4c-.013.75.198 1.487.606 2.117L7.734 14h.533l3.147-6.383c.409-.63.62-1.367.606-2.117a4.056 4.056 0 0 0-1.188-2.812zM7.925 2.5l.082.01.074-.01a3.075 3.075 0 0 1 2.941 3.037 2.74 2.74 0 0 1-.467 1.568l-.02.034-.017.035L8 12.279l-2.517-5.1-.017-.039-.02-.034a2.74 2.74 0 0 1-.467-1.568A3.074 3.074 0 0 1 7.924 2.5zm.612 2.169a1 1 0 1 0-1.112 1.663 1 1 0 0 0 1.112-1.663zM6.87 3.837a2 2 0 1 1 2.22 3.326 2 2 0 0 1-2.22-3.326z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLockSmall;
impl IconShape for VsLockSmall {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 8L4 7H12L13 8V13L12 14H4L3 13V8ZM4 8V13H12V8H4Z",
            }
            path {
                d: "M11 7V5C11 3.34315 9.65685 2 8 2C6.34315 2 5 3.34315 5 5V7H6V5C6 3.89543 6.89543 3 8 3C9.10457 3 10 3.89543 10 5V7H11Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsLock;
impl IconShape for VsLock {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 7h-1V5a4 4 0 1 0-8 0v2H3L2 8v6l1 1h10l1-1V8l-1-1zM5 5a3 3 0 1 1 6 0v2H5V5zm8 9H3V8h10v6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMagnet;
impl IconShape for VsMagnet {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 1.5c-3.9 0-7 3.1-7 7v5l1 1h3l1-1v-5c0-1.1.9-2 2-2s2 .9 2 2v5l1 1h3l1-1v-5c0-3.9-3.1-7-7-7zm-3 12H2v-3h3v3zm9 0h-3v-3h3v3zm-3-4v-1c0-1.7-1.3-3-3-3-1.6 0-2.9 1.3-3 2.8v1.2H2v-1c0-3.3 2.7-6 6-6s6 2.7 6 6v1h-3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMailRead;
impl IconShape for VsMailRead {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.25 1.57h-.51L1 5.56v7.94l.5.5h13l.5-.5V5.56L8.25 1.57zM8 2.58l5.63 3.32-1.37 1.59H3.74L2.43 5.9 8 2.58zM14 13H2V6.92L3.11 8.3l.39.19h9l.39-.19L14 6.92V13z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMail;
impl IconShape for VsMail {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1 3.5l.5-.5h13l.5.5v9l-.5.5h-13l-.5-.5v-9zm1 1.035V12h12V4.536L8.31 8.9H7.7L2 4.535zM13.03 4H2.97L8 7.869 13.03 4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMapFilled;
impl IconShape for VsMapFilled {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 5.5V13L5.5 10.8125V3.3125L2 5.5Z",
            }
            path {
                d: "M9.5 12.6875V5.1875L6.5 3.3125V10.8125L9.5 12.6875Z",
            }
            path {
                d: "M10.5 12.6875V5.1875L14 3V10.5L10.5 12.6875Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMapVerticalFilled;
impl IconShape for VsMapVerticalFilled {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.5 2L13 2L10.8125 5.5L3.3125 5.5L5.5 2Z",
            }
            path {
                d: "M12.6875 9.5L5.1875 9.5L3.3125 6.5L10.8125 6.5L12.6875 9.5Z",
            }
            path {
                d: "M12.6875 10.5L5.1875 10.5L3 14L10.5 14L12.6875 10.5Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMapVertical;
impl IconShape for VsMapVertical {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.77711 2.49976L12.0979 2.49976L10.2229 5.49976L3.90211 5.49976L5.77711 2.49976ZM3.90211 6.49976L10.2229 6.49976L12.0979 9.49976L5.77711 9.49976L3.90211 6.49976ZM11.0896 5.99976L13.424 2.26475L13 1.49976L5.49999 1.49976L5.07599 1.73476L2.57599 5.73476L2.57599 6.26476L4.91036 9.99976L2.57599 13.7348L2.99999 14.4998L10.5 14.4998L10.924 14.2648L13.424 10.2648L13.424 9.73476L11.0896 5.99976ZM5.77711 10.4998L12.0979 10.4998L10.2229 13.4998L3.90211 13.4998L5.77711 10.4998Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMap;
impl IconShape for VsMap {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.5 5.77705V12.0978L5.5 10.2228V3.90205L2.5 5.77705ZM6.5 3.90205V10.2228L9.5 12.0978V5.77705L6.5 3.90205ZM6 11.0896L2.265 13.4239L1.5 12.9999V5.49993L1.735 5.07593L5.735 2.57593H6.265L10 4.9103L13.735 2.57593L14.5 2.99993V10.4999L14.265 10.9239L10.265 13.4239H9.735L6 11.0896ZM10.5 5.77705V12.0978L13.5 10.2228V3.90205L10.5 5.77705Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMarkdown;
impl IconShape for VsMarkdown {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.345 5h2.1v6.533H6.993l.055-5.31-1.774 5.31H4.072l-1.805-5.31c.04.644.06 5.31.06 5.31H1V5h2.156s1.528 4.493 1.577 4.807L6.345 5zm6.71 3.617v-3.5H11.11v3.5H9.166l2.917 2.916L15 8.617h-1.945z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMegaphone;
impl IconShape for VsMegaphone {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 6.77l12.33-3.43.67.53v8.6l-.67.53-6.089-1.595a2.16 2.16 0 1 1-4.178-1.095L2 9.77l-.42-.53V7.3L2 6.77zm3.006 3.787a1.13 1.13 0 0 0-.04.242 1.17 1.17 0 0 0 2.288.347l-2.248-.589zM2.58 8.82L14 11.83V4.5L2.58 7.72v1.1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMention;
impl IconShape for VsMention {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.75409 10.082H9.71709C9.62501 10.6077 9.34889 11.0835 8.93813 11.4243C8.52737 11.7651 8.00876 11.9486 7.47509 11.942C7.14645 11.9529 6.81979 11.8872 6.52093 11.7501C6.22207 11.613 5.95919 11.4082 5.75309 11.152C5.29076 10.5433 5.05937 9.79032 5.10009 9.02702C5.06003 7.97574 5.37727 6.9419 6.00009 6.09402C6.27187 5.72239 6.62957 5.42205 7.04262 5.21866C7.45567 5.01527 7.91181 4.91487 8.37209 4.92602C8.72661 4.92177 9.07449 5.02231 9.37209 5.21502C9.64413 5.37906 9.84577 5.63806 9.93809 5.94202H9.96909C9.98009 5.79602 10.0051 5.49602 10.0471 5.04202H11.0241C10.7788 7.91202 10.6564 9.37302 10.6571 9.42502C10.6571 10.503 10.9668 11.042 11.5861 11.042C11.8803 11.0305 12.1658 10.9394 12.4124 10.7785C12.6589 10.6175 12.8572 10.3927 12.9861 10.128C13.3864 9.40541 13.5795 8.58634 13.5441 7.76102C13.5693 7.11522 13.4624 6.47108 13.2299 5.86804C12.9974 5.26501 12.6443 4.71579 12.1921 4.25402C11.6889 3.78464 11.0957 3.4222 10.4483 3.18861C9.80104 2.95502 9.11309 2.85515 8.42609 2.89502C7.70766 2.87569 6.99344 3.01075 6.33168 3.29109C5.66991 3.57142 5.07601 3.9905 4.59009 4.52002C3.57231 5.62318 3.02829 7.08181 3.07509 8.58202C3.04367 9.29097 3.15514 9.999 3.40286 10.664C3.65059 11.329 4.02951 11.9374 4.51709 12.453C5.0285 12.9532 5.63778 13.3422 6.30668 13.5957C6.97558 13.8491 7.68965 13.9616 8.40409 13.926C9.53781 13.9572 10.6645 13.7384 11.7041 13.285V14.254C10.6292 14.671 9.48146 14.8675 8.32909 14.832C7.48919 14.8696 6.65036 14.7371 5.86297 14.4424C5.07558 14.1477 4.35588 13.6969 3.74709 13.117C3.16356 12.5291 2.70767 11.827 2.40793 11.0547C2.10818 10.2825 1.97104 9.4567 2.00509 8.62902C1.97746 7.75691 2.12461 6.88808 2.43786 6.07369C2.75111 5.2593 3.22414 4.51582 3.82909 3.88702C4.43161 3.26559 5.15744 2.77704 5.95997 2.45275C6.7625 2.12847 7.62398 1.97562 8.48909 2.00402C10.1038 1.94632 11.679 2.51124 12.8891 3.58202C13.4621 4.10066 13.9148 4.73836 14.2154 5.45041C14.5159 6.16246 14.6572 6.93164 14.6291 7.70402C14.6758 8.79297 14.3546 9.86595 13.7171 10.75C13.4673 11.1142 13.1334 11.4127 12.7436 11.6203C12.3538 11.8278 11.9197 11.9382 11.4781 11.942C10.3348 11.942 9.76009 11.322 9.75409 10.082ZM8.35409 5.83202C8.03474 5.83297 7.72139 5.91886 7.44619 6.08089C7.17099 6.24292 6.94386 6.47524 6.78809 6.75402C6.36861 7.44196 6.16001 8.23777 6.18809 9.04302C6.15844 9.55812 6.29899 10.0687 6.58809 10.496C6.71024 10.6656 6.8721 10.8027 7.05951 10.8953C7.24692 10.9879 7.45415 11.0331 7.66309 11.027C7.9833 11.0251 8.29661 10.9338 8.56778 10.7635C8.83896 10.5932 9.05724 10.3506 9.19809 10.063C9.61196 9.27571 9.80936 8.39262 9.77009 7.50402C9.77009 6.39002 9.29875 5.83269 8.35609 5.83202H8.35409Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMenu;
impl IconShape for VsMenu {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 5H0V4h16v1zm0 8H0v-1h16v1zm0-4.008H0V8h16v.992z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMerge;
impl IconShape for VsMerge {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.5 4.646L8.354 2.5h-.707L5.5 4.646l.707.707L7.3 4.261V5.28h-.02v.456l.025.001.006.319c.004.187.02.379.05.574.03.195.069.39.117.586.048.195.114.404.2.627.155.379.343.722.565 1.031.221.309.46.598.715.867.255.27.508.535.76.797.25.262.478.541.681.838.203.297.368.621.494.973.125.351.188.755.188 1.213v.884H12.5v-.884a5.991 5.991 0 0 0-.166-1.39 4.638 4.638 0 0 0-.427-1.1 5.875 5.875 0 0 0-.604-.897c-.222-.27-.453-.527-.693-.774-.24-.246-.471-.492-.693-.738a6.39 6.39 0 0 1-.604-.785 3.794 3.794 0 0 1-.433-.914 3.676 3.676 0 0 1-.16-1.13V5.28h-.001v-1l1.074 1.074.707-.708zM7.042 9.741a8.19 8.19 0 0 0 .329-.369 6.06 6.06 0 0 1-.62-1.15L6.744 8.2a7.26 7.26 0 0 1-.095-.263c-.17.256-.359.498-.565.726-.222.246-.453.492-.693.738-.24.247-.47.504-.693.774-.221.27-.423.568-.604.896a4.643 4.643 0 0 0-.427 1.102 5.995 5.995 0 0 0-.166 1.389v.884h1.42v-.884c0-.457.062-.862.188-1.213.125-.352.29-.676.493-.973.203-.297.43-.576.682-.838.251-.262.504-.527.76-.797z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMicFilled;
impl IconShape for VsMicFilled {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.93994 10.5C8.60298 10.5 9.23887 10.2366 9.70771 9.76777C10.1765 9.29893 10.4399 8.66304 10.4399 8V3.5C10.4399 2.83696 10.1765 2.20107 9.70771 1.73223C9.23887 1.26339 8.60298 1 7.93994 1C7.2769 1 6.64102 1.26339 6.17217 1.73223C5.70333 2.20107 5.43994 2.83696 5.43994 3.5V8C5.43994 8.66304 5.70333 9.29893 6.17217 9.76777C6.64102 10.2366 7.2769 10.5 7.93994 10.5ZM8.43994 12.472V14H10.4399V15H5.43994V14H7.43994V12.472C6.33992 12.349 5.32381 11.8249 4.58588 10.9999C3.84796 10.1749 3.43997 9.10688 3.43994 8H4.43994C4.43994 8.92826 4.80869 9.8185 5.46507 10.4749C6.12145 11.1313 7.01168 11.5 7.93994 11.5C8.8682 11.5 9.75844 11.1313 10.4148 10.4749C11.0712 9.8185 11.4399 8.92826 11.4399 8H12.4399C12.4399 9.10688 12.0319 10.1749 11.294 10.9999C10.5561 11.8249 9.53996 12.349 8.43994 12.472Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMic;
impl IconShape for VsMic {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 10.5C8.66304 10.5 9.29893 10.2366 9.76777 9.76777C10.2366 9.29893 10.5 8.66304 10.5 8V3.5C10.5 2.83696 10.2366 2.20107 9.76777 1.73223C9.29893 1.26339 8.66304 1 8 1C7.33696 1 6.70107 1.26339 6.23223 1.73223C5.76339 2.20107 5.5 2.83696 5.5 3.5V8C5.5 8.66304 5.76339 9.29893 6.23223 9.76777C6.70107 10.2366 7.33696 10.5 8 10.5ZM6.5 3.5C6.5 3.10218 6.65804 2.72064 6.93934 2.43934C7.22064 2.15804 7.60218 2 8 2C8.39782 2 8.77936 2.15804 9.06066 2.43934C9.34196 2.72064 9.5 3.10218 9.5 3.5V8C9.5 8.39782 9.34196 8.77936 9.06066 9.06066C8.77936 9.34196 8.39782 9.5 8 9.5C7.60218 9.5 7.22064 9.34196 6.93934 9.06066C6.65804 8.77936 6.5 8.39782 6.5 8V3.5ZM8.5 12.472V14H10.5V15H5.5V14H7.5V12.472C6.39998 12.349 5.38387 11.8249 4.64594 10.9999C3.90801 10.1749 3.50003 9.10688 3.5 8H4.5C4.5 8.92826 4.86875 9.8185 5.52513 10.4749C6.1815 11.1313 7.07174 11.5 8 11.5C8.92826 11.5 9.8185 11.1313 10.4749 10.4749C11.1313 9.8185 11.5 8.92826 11.5 8H12.5C12.5 9.10688 12.092 10.1749 11.3541 10.9999C10.6161 11.8249 9.60002 12.349 8.5 12.472V12.472Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMilestone;
impl IconShape for VsMilestone {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8 1H7v2H1.5l-.5.5v4l.5.5H7v7h1V8h4.49l.34-.13 2.18-2v-.74l-2.18-2L12.5 3H8V1zm4.29 6H2V4h10.29l1.63 1.5L12.29 7zM5 5h5v1H5V5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMirror;
impl IconShape for VsMirror {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.57 1l6.2 4 .23.38v9.2l-.76.42L8 11l-6.24 4-.76-.42v-9.2L1.23 5l6.2-4h1.14zm-.06 9.13L14 13.67V5.65l-5.49-3.5V5h-1V2.13L2 5.67v8l5.51-3.56v.02h1zm.9-4.78l.71-.7 2.47 2.48v.71l-2.46 2.46-.7-.7L11.02 8h-6L6.6 9.6l-.7.7-2.46-2.46v-.71l2.48-2.48.7.7L4.98 7h6.08L9.41 5.35z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMortarBoard;
impl IconShape for VsMortarBoard {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 5.66L8.18 3h-.36L1 5.66V12h1V7l2.31.9a4.35 4.35 0 0 0-.79 2.48c-.01.11-.01.22 0 .33v.11l.28.4L7.78 13h.41l3.94-1.81.28-.4v-.44a4.39 4.39 0 0 0-.78-2.47L15 6.57v-.91zm-3.52 4.68v.07L8 12l-3.5-1.6a.13.13 0 0 1 0-.06 3.44 3.44 0 0 1 .75-2.12l2.58 1h.36l2.56-1a3.4 3.4 0 0 1 .73 2.12zM8 8.25L2.52 6.12 8 4l5.48 2.14L8 8.25z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMove;
impl IconShape for VsMove {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.232 10.707L8.5 12.44V9h-1v3.44l-1.732-1.733-.707.707L7.646 14h.708l2.585-2.586-.707-.707zM5.061 3.586l.707.707L7.5 2.56V6h1V2.56l1.732 1.733.707-.707L8.354 1h-.708L5.061 3.586zm-.268 1.682L3.06 7H6.5v1H3.06l1.733 1.732-.707.707L1.5 7.854v-.708l2.586-2.585.707.707zM9.5 7h3.44l-1.733-1.732.707-.707L14.5 7.146v.708l-2.586 2.585-.707-.707L12.94 8H9.5V7z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMultipleWindows;
impl IconShape for VsMultipleWindows {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 1.5l.5-.5h8l.5.5v7l-.5.5H12V8h2V4H7v1H6V1.5zM7 2v1h7V2H7zM1.5 7l-.5.5v7l.5.5h8l.5-.5v-7L9.5 7h-8zM2 9V8h7v1H2zm0 1h7v4H2v-4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMusic;
impl IconShape for VsMusic {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14 7H13V9.49982C12.5822 9.18597 12.0628 9 11.5 9C10.1193 9 9 10.1193 9 11.5C9 12.8807 10.1193 14 11.5 14C12.8807 14 14 12.8807 14 11.5V7ZM11.5 10C12.3284 10 13 10.6716 13 11.5C13 12.3284 12.3284 13 11.5 13C10.6716 13 10 12.3284 10 11.5C10 10.6716 10.6716 10 11.5 10Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M13.4688 2.00098L5.46881 2.50098L5 3V10.4998C4.58217 10.186 4.0628 10 3.5 10C2.11929 10 1 11.1193 1 12.5C1 13.8807 2.11929 15 3.5 15C4.88071 15 6 13.8807 6 12.5V6.46974L13 6.03224V7H14V2.5L13.4688 2.00098ZM13 3.03223V5.03029L6 5.46779V3.46973L13 3.03223ZM3.5 11C4.32843 11 5 11.6716 5 12.5C5 13.3284 4.32843 14 3.5 14C2.67157 14 2 13.3284 2 12.5C2 11.6716 2.67157 11 3.5 11Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsMute;
impl IconShape for VsMute {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 5h2.79l3.86-3.83.85.35v13l-.85.33L4.29 11H1.5l-.5-.5v-5l.5-.5zm3.35 5.17L8 13.31V2.73L4.85 5.85 4.5 6H2v4h2.5l.35.17zm9.381-4.108l.707.707L13.207 8.5l1.731 1.732-.707.707L12.5 9.207l-1.732 1.732-.707-.707L11.793 8.5 10.06 6.77l.707-.707 1.733 1.73 1.731-1.731z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsNewFile;
impl IconShape for VsNewFile {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.5 1.1l3.4 3.5.1.4v2h-1V6H8V2H3v11h4v1H2.5l-.5-.5v-12l.5-.5h6.7l.3.1zM9 2v3h2.9L9 2zm4 14h-1v-3H9v-1h3V9h1v3h3v1h-3v3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsNewFolder;
impl IconShape for VsNewFolder {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.5 2H7.71l-.85-.85L6.51 1h-5l-.5.5v11l.5.5H7v-1H1.99V6h4.49l.35-.15.86-.86H14v1.5l-.001.51h1.011V2.5L14.5 2zm-.51 2h-6.5l-.35.15-.86.86H2v-3h4.29l.85.85.36.15H14l-.01.99zM13 16h-1v-3H9v-1h3V9h1v3h3v1h-3v3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsNewline;
impl IconShape for VsNewline {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12 5.5V7.484C12 7.61661 11.9473 7.74379 11.8535 7.83755C11.7598 7.93132 11.6326 7.984 11.5 7.984H4.618L6.251 6.351L5.544 5.644L3.423 7.765L3 8.188V8.756L5.544 11.3L6.251 10.593L4.641 8.984H11.5C11.8978 8.984 12.2793 8.82597 12.5607 8.54466C12.842 8.26336 13 7.88182 13 7.484V5.5H12Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsNoNewline;
impl IconShape for VsNoNewline {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.333 5.506a3 3 0 1 1 3.334 4.989 3 3 0 0 1-3.334-4.99zm2.677.777A1.986 1.986 0 0 0 2 8.009c.004.353.102.698.283 1.001L5.01 6.283zM2.99 9.717A1.986 1.986 0 0 0 6 7.991a1.988 1.988 0 0 0-.283-1.001L2.99 9.717zM14 5v1.984a.5.5 0 0 1-.5.5H9.367L11 5.851l-.707-.707-2.121 2.121-.423.423v.568l2.544 2.544.707-.707-1.61-1.609h4.11a1.5 1.5 0 0 0 1.5-1.5V5h-1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsNote;
impl IconShape for VsNote {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 2h13l.5.5v10l-.5.5h-13l-.5-.5v-10l.5-.5zM2 3v9h12V3H2zm2 2h8v1H4V5zm6 2H4v1h6V7zM4 9h4v1H4V9z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsNotebookTemplate;
impl IconShape for VsNotebookTemplate {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1 5H0V4h1v1zm0 2H0V6h1v1zm0 2H0V8h1v1zm0 2H0v-1h1v1zm0 2H0v-1h1v1zm0 1v1H0v-1h1zm0 1h1v1H1v-1zm2 0h1v1H3v-1zM1 1H0V0h1v1zm2 0H2V0h1v1zm1-1h1v1H4V0zm3 1H6V0h1v1zm2 0H8V0h1v1zm2 0h-1V0h1v1zm0 1V1h1v1h-1zm1 2h-1V3h1v1zM1 3H0V2h1v1z",
            }
            path {
                clip_rule: "evenodd",
                d: "M5 6l1-1h7l1 1v9l-1 1H6l-1-1V6zm1 0v9h7V6H6z",
                fill_rule: "evenodd",
            }
            path {
                d: "M15 7h1v2h-1V7zm0 3h1v2h-1v-2zm0 3h1v2h-1v-2zM7 8h5v1H7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsNotebook;
impl IconShape for VsNotebook {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 2l1-1h9l1 1v12l-1 1H3l-1-1V2zm1 0v12h9V2H3zm1 2l1-1h5l1 1v1l-1 1H5L4 5V4zm1 0v1h5V4H5zm10 1h-1v2h1V5zm-1 3h1v2h-1V8zm1 3h-1v2h1v-2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsOctoface;
impl IconShape for VsOctoface {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13.863 5.673c.113-.28.48-1.392-.114-2.897 0 0-.919-.288-3.01 1.138-.875-.245-1.812-.28-2.739-.28-.928 0-1.864.035-2.739.28-2.091-1.435-3.01-1.138-3.01-1.138-.595 1.505-.227 2.617-.113 2.897C1.428 6.433 1 7.413 1 8.603c0 4.507 2.914 5.522 6.982 5.522 4.07 0 7.018-1.015 7.018-5.521 0-1.19-.429-2.17-1.137-2.931zM8 13.268c-2.888 0-5.232-.132-5.232-2.932 0-.665.332-1.295.892-1.811.936-.857 2.537-.402 4.34-.402 1.811 0 3.395-.455 4.34.402.569.516.893 1.138.893 1.811 0 2.791-2.346 2.931-5.233 2.931zM5.804 8.883c-.578 0-1.05.7-1.05 1.557 0 .858.472 1.566 1.05 1.566.577 0 1.05-.7 1.05-1.566 0-.866-.473-1.557-1.05-1.557zm4.392 0c-.577 0-1.05.691-1.05 1.557s.473 1.566 1.05 1.566c.578 0 1.05-.7 1.05-1.566 0-.866-.463-1.557-1.05-1.557z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsOpenPreview;
impl IconShape for VsOpenPreview {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 1h11l1 1v5.3a3.21 3.21 0 0 0-1-.3V2H9v10.88L7.88 14H3l-1-1V2l1-1zm0 12h5V2H3v11zm10.379-4.998a2.53 2.53 0 0 0-1.19.348h-.03a2.51 2.51 0 0 0-.799 3.53L9 14.23l.71.71 2.35-2.36c.325.22.7.358 1.09.4a2.47 2.47 0 0 0 1.14-.13 2.51 2.51 0 0 0 1-.63 2.46 2.46 0 0 0 .58-1 2.63 2.63 0 0 0 .07-1.15 2.53 2.53 0 0 0-1.35-1.81 2.53 2.53 0 0 0-1.211-.258zm.24 3.992a1.5 1.5 0 0 1-.979-.244 1.55 1.55 0 0 1-.56-.68 1.49 1.49 0 0 1-.08-.86 1.49 1.49 0 0 1 1.18-1.18 1.49 1.49 0 0 1 .86.08c.276.117.512.311.68.56a1.5 1.5 0 0 1-1.1 2.324z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsOrganization;
impl IconShape for VsOrganization {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.111 4.663A2 2 0 1 1 6.89 1.337a2 2 0 0 1 2.222 3.326zm-.555-2.494A1 1 0 1 0 7.444 3.83a1 1 0 0 0 1.112-1.66zm2.61.03a1.494 1.494 0 0 1 1.895.188 1.513 1.513 0 0 1-.487 2.46 1.492 1.492 0 0 1-1.635-.326 1.512 1.512 0 0 1 .228-2.321zm.48 1.61a.499.499 0 1 0 .705-.708.509.509 0 0 0-.351-.15.499.499 0 0 0-.5.503.51.51 0 0 0 .146.356zM3.19 12.487H5v1.005H3.19a1.197 1.197 0 0 1-.842-.357 1.21 1.21 0 0 1-.348-.85v-1.81a.997.997 0 0 1-.71-.332A1.007 1.007 0 0 1 1 9.408V7.226c.003-.472.19-.923.52-1.258.329-.331.774-.52 1.24-.523H4.6a2.912 2.912 0 0 0-.55 1.006H2.76a.798.798 0 0 0-.54.232.777.777 0 0 0-.22.543v2.232h1v2.826a.202.202 0 0 0 .05.151.24.24 0 0 0 .14.05zm7.3-6.518a1.765 1.765 0 0 0-1.25-.523H6.76a1.765 1.765 0 0 0-1.24.523c-.33.335-.517.786-.52 1.258v3.178a1.06 1.06 0 0 0 .29.734 1 1 0 0 0 .71.332v2.323a1.202 1.202 0 0 0 .35.855c.18.168.407.277.65.312h2a1.15 1.15 0 0 0 1-1.167V11.47a.997.997 0 0 0 .71-.332 1.006 1.006 0 0 0 .29-.734V7.226a1.8 1.8 0 0 0-.51-1.258zM10 10.454H9v3.34a.202.202 0 0 1-.06.14.17.17 0 0 1-.14.06H7.19a.21.21 0 0 1-.2-.2v-3.34H6V7.226c0-.203.079-.398.22-.543a.798.798 0 0 1 .54-.232h2.48a.778.778 0 0 1 .705.48.748.748 0 0 1 .055.295v3.228zm2.81 3.037H11v-1.005h1.8a.24.24 0 0 0 .14-.05.2.2 0 0 0 .06-.152V9.458h1V7.226a.777.777 0 0 0-.22-.543.798.798 0 0 0-.54-.232h-1.29a2.91 2.91 0 0 0-.55-1.006h1.84a1.77 1.77 0 0 1 1.24.523c.33.335.517.786.52 1.258v2.182c0 .273-.103.535-.289.733-.186.199-.44.318-.711.333v1.81c0 .319-.125.624-.348.85a1.197 1.197 0 0 1-.842.357zM4 1.945a1.494 1.494 0 0 0-1.386.932A1.517 1.517 0 0 0 2.94 4.52 1.497 1.497 0 0 0 5.5 3.454c0-.4-.158-.784-.44-1.067A1.496 1.496 0 0 0 4 1.945zm0 2.012a.499.499 0 0 1-.5-.503.504.504 0 0 1 .5-.503.509.509 0 0 1 .5.503.504.504 0 0 1-.5.503z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsOutput;
impl IconShape for VsOutput {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M19.5 0v1.5L21 3v19.5L19.5 24h-15L3 22.5V3l1.5-1.5V0H6v1.5h3V0h1.5v1.5h3V0H15v1.5h3V0h1.5zm-15 22.5h15V3h-15v19.5zM7.5 6h9v1.5h-9V6zm9 6h-9v1.5h9V12zm-9 6h9v1.5h-9V18z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPackage;
impl IconShape for VsPackage {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.61 3l5.74 1.53L15 5v6.74l-.37.48-6.13 1.69-6.14-1.69-.36-.48V5l.61-.47L8.34 3h.27zm-.09 1l-4 1 .55.2 3.43.9 3-.81.95-.29-3.93-1zM3 11.36l5 1.37V7L3 5.66v5.7zM9 7v5.73l5-1.37V5.63l-2.02.553V8.75l-1 .26V6.457L9 7z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPaintcan;
impl IconShape for VsPaintcan {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.54 11.811l-1.14-3.12v-.06l-4.91-4.91v-1.24a1.66 1.66 0 0 0-.11-.58 1.48 1.48 0 0 0-.83-.8 1.42 1.42 0 0 0-.58-.1 1.47 1.47 0 0 0-1.48 1.48v3.26l-3.06 3a1.52 1.52 0 0 0 0 2.12l3.63 3.63c.14.141.307.253.49.33a1.53 1.53 0 0 0 1.14 0 1.51 1.51 0 0 0 .49-.33l4.93-4.92-.66 2.2a1.19 1.19 0 0 0 0 .46c.033.152.098.296.19.42.098.121.216.223.35.3.14.07.294.11.45.12a1 1 0 0 0 .48-.09 1.14 1.14 0 0 0 .39-.29.98.98 0 0 0 .22-.44c.032-.145.035-.294.01-.44zm-8-9.33a.46.46 0 0 1 0-.2.52.52 0 0 1 .12-.17.64.64 0 0 1 .18-.1.5.5 0 0 1 .21 0 .5.5 0 0 1 .32.15.5.5 0 0 1 .12.33v1.26l-1 1 .05-2.27zm1 11.35a.36.36 0 0 1-.16.11.47.47 0 0 1-.38 0 .361.361 0 0 1-.16-.11l-3.63-3.62a.5.5 0 0 1 0-.71l4.35-4.35v2.85a.74.74 0 0 0-.24.55.75.75 0 1 0 1.17-.55v-2.83l3.85 3.87-4.8 4.79z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPassFilled;
impl IconShape for VsPassFilled {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8 15A7 7 0 1 0 8 1a7 7 0 0 0 0 14zm-1.02-4.13h-.71L4 8.6l.71-.71 1.92 1.92 4.2-4.21.71.71-4.56 4.56z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPass;
impl IconShape for VsPass {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.27 10.87h.71l4.56-4.56-.71-.71-4.2 4.21-1.92-1.92L4 8.6l2.27 2.27z",
            }
            path {
                clip_rule: "evenodd",
                d: "M8.6 1c1.6.1 3.1.9 4.2 2 1.3 1.4 2 3.1 2 5.1 0 1.6-.6 3.1-1.6 4.4-1 1.2-2.4 2.1-4 2.4-1.6.3-3.2.1-4.6-.7-1.4-.8-2.5-2-3.1-3.5C.9 9.2.8 7.5 1.3 6c.5-1.6 1.4-2.9 2.8-3.8C5.4 1.3 7 .9 8.6 1zm.5 12.9c1.3-.3 2.5-1 3.4-2.1.8-1.1 1.3-2.4 1.2-3.8 0-1.6-.6-3.2-1.7-4.3-1-1-2.2-1.6-3.6-1.7-1.3-.1-2.7.2-3.8 1-1.1.8-1.9 1.9-2.3 3.3-.4 1.3-.4 2.7.2 4 .6 1.3 1.5 2.3 2.7 3 1.2.7 2.6.9 3.9.6z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPercentage;
impl IconShape for VsPercentage {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3.60962 12.6877L11.6096 2.68765L12.3905 3.31235L4.39049 13.3123L3.60962 12.6877Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M4.5 6.5C3.67157 6.5 3 5.82843 3 5C3 4.17157 3.67157 3.5 4.5 3.5C5.32843 3.5 6 4.17157 6 5C6 5.82843 5.32843 6.5 4.5 6.5ZM4.5 7.5C5.88071 7.5 7 6.38071 7 5C7 3.61929 5.88071 2.5 4.5 2.5C3.11929 2.5 2 3.61929 2 5C2 6.38071 3.11929 7.5 4.5 7.5Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M11.5 12.5C10.6716 12.5 10 11.8284 10 11C10 10.1716 10.6716 9.5 11.5 9.5C12.3284 9.5 13 10.1716 13 11C13 11.8284 12.3284 12.5 11.5 12.5ZM11.5 13.5C12.8807 13.5 14 12.3807 14 11C14 9.61929 12.8807 8.5 11.5 8.5C10.1193 8.5 9 9.61929 9 11C9 12.3807 10.1193 13.5 11.5 13.5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPersonAdd;
impl IconShape for VsPersonAdd {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13 10h-1v2h-2v1h2v2h1v-2h2v-1h-2v-2zM8.556 2.169a1 1 0 1 0-1.112 1.663 1 1 0 0 0 1.112-1.663zm-1.667-.832A2 2 0 1 1 9.11 4.663a2 2 0 0 1-2.22-3.326zM6.77 5.49h2.46A1.77 1.77 0 0 1 11 7.26V8h-1v-.74a.76.76 0 0 0-.77-.77H6.77a.76.76 0 0 0-.77.77V10h1v3.31a.2.2 0 0 0 .2.2H8v1.02h-.8a1.2 1.2 0 0 1-1.2-1.2V11a1.06 1.06 0 0 1-1-1.1V7.26a1.77 1.77 0 0 1 1.77-1.77z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPerson;
impl IconShape for VsPerson {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 2a1 1 0 1 1 0 2 1 1 0 0 1 0-2zm0-1a2 2 0 1 0 0 4 2 2 0 0 0 0-4zm1.23 4.49H6.77A1.77 1.77 0 0 0 5 7.26V9.9A1.06 1.06 0 0 0 6 11v2.33a1.2 1.2 0 0 0 1.2 1.2h1.6a1.2 1.2 0 0 0 1.2-1.24V11a1.06 1.06 0 0 0 1-1.1V7.26a1.77 1.77 0 0 0-1.77-1.77zM6 10V7.26a.76.76 0 0 1 .77-.77h2.46a.76.76 0 0 1 .77.77V10H9v3.31a.2.2 0 0 1-.2.2H7.2a.2.2 0 0 1-.2-.2V10H6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPiano;
impl IconShape for VsPiano {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1 2L0 3V9H1V3H3V8.5L3.5 9H4.5L5 8.5V3H7V8.5L7.5 9H8.5L9 8.5V3H11V8.5L11.5 9H12.5L13 8.5V3H15V13H12V10H11V13H8.5V10H7.5V13H5V10H4V13H1V9H0V13L1 14H15L16 13V3L15 2H1Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPieChart;
impl IconShape for VsPieChart {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 6H13.9C13.5023 4.04087 11.9591 2.4977 10 2.10002V6ZM10 1.08296C12.5125 1.50448 14.4955 3.4875 14.917 6C14.9716 6.32521 15 6.65929 15 7H9V1C9.34071 1 9.67479 1.0284 10 1.08296ZM7 8.00003L8 9.00003H12.9C12.4367 11.2823 10.4189 13 8 13C5.23858 13 3 10.7614 3 8C3 5.58104 4.71776 3.56329 7 3.10002V8.00003ZM8 14C10.973 14 13.4409 11.8377 13.917 9.00003C13.9716 8.67482 14 8.34074 14 8.00003H8V2C7.65929 2 7.32521 2.0284 7 2.08296C4.16229 2.55904 2 5.027 2 8C2 11.3137 4.68629 14 8 14Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPin;
impl IconShape for VsPin {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 5v7h-.278c-.406 0-.778-.086-1.117-.258A2.528 2.528 0 0 1 11.73 11H8.87a3.463 3.463 0 0 1-.546.828 3.685 3.685 0 0 1-.735.633c-.27.177-.565.31-.882.398a3.875 3.875 0 0 1-.985.141h-.5V9H2l-1-.5L2 8h3.222V4h.5c.339 0 .664.047.977.14.312.094.607.227.883.4A3.404 3.404 0 0 1 8.87 6h2.859a2.56 2.56 0 0 1 .875-.734c.338-.172.71-.26 1.117-.266H14zm-.778 1.086a1.222 1.222 0 0 0-.32.156 1.491 1.491 0 0 0-.43.461L12.285 7H8.183l-.117-.336a2.457 2.457 0 0 0-.711-1.047C7.027 5.331 6.427 5.09 6 5v7c.427-.088 1.027-.33 1.355-.617.328-.287.565-.636.71-1.047L8.184 10h4.102l.18.297c.057.094.122.177.195.25.073.073.153.143.242.21.088.069.195.12.32.157V6.086z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPinnedDirty;
impl IconShape for VsPinnedDirty {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 2h7v.278c0 .406-.086.778-.258 1.117-.172.339-.42.63-.742.875v2.86c.307.145.583.328.828.546a3.7 3.7 0 0 1 .54.598 4.92 4.92 0 0 0-.896.412l-.007.004-.03.018a2.456 2.456 0 0 0-1.099-.774L9 7.817V3.715l.297-.18c.094-.057.177-.122.25-.195a2.28 2.28 0 0 0 .21-.242.968.968 0 0 0 .157-.32H5.086c.042.125.094.232.156.32a1.494 1.494 0 0 0 .461.43L6 3.715v4.102l-.336.117c-.411.146-.76.383-1.047.711C4.331 8.973 4.09 9.573 4 10h5.002a5.025 5.025 0 0 0-.481.778H8V14l-.5 1-.5-1v-3.222H3v-.5c0-.339.047-.664.14-.977.094-.312.227-.607.4-.883A3.404 3.404 0 0 1 5 7.13V4.27a2.561 2.561 0 0 1-.734-.875A2.505 2.505 0 0 1 4 2.278V2zm7.485 8.41a2.924 2.924 0 0 1 .718-.302c.256-.072.522-.108.797-.108s.541.036.797.108a2.956 2.956 0 0 1 1.321.773 2.956 2.956 0 0 1 .774 1.322c.072.256.108.522.108.797s-.036.541-.108.797a2.953 2.953 0 0 1-.774 1.324 3.013 3.013 0 0 1-1.321.774c-.256.07-.522.105-.797.105s-.541-.035-.797-.105a3.037 3.037 0 0 1-1.324-.774 3.037 3.037 0 0 1-.773-1.324A2.994 2.994 0 0 1 10 13c0-.275.035-.541.105-.797a3.013 3.013 0 0 1 .883-1.425c.154-.14.32-.262.497-.368z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPinned;
impl IconShape for VsPinned {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 2h7v.278c0 .406-.086.778-.258 1.117-.172.339-.42.63-.742.875v2.86c.307.145.583.328.828.546.245.219.456.464.633.735.177.27.31.565.398.882.089.318.136.646.141.985v.5H8V14l-.5 1-.5-1v-3.222H3v-.5c0-.339.047-.664.14-.977.094-.312.227-.607.4-.883A3.404 3.404 0 0 1 5 7.13V4.27a2.561 2.561 0 0 1-.734-.875A2.505 2.505 0 0 1 4 2.278V2zm1.086.778c.042.125.094.232.156.32a1.494 1.494 0 0 0 .461.43L6 3.715v4.102l-.336.117c-.411.146-.76.383-1.047.711C4.331 8.973 4.09 9.573 4 10h7c-.088-.427-.33-1.027-.617-1.355a2.456 2.456 0 0 0-1.047-.71L9 7.816V3.715l.297-.18c.094-.057.177-.122.25-.195a2.28 2.28 0 0 0 .21-.242.968.968 0 0 0 .157-.32H5.086z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPlayCircle;
impl IconShape for VsPlayCircle {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.6 1c1.6.1 3.1.9 4.2 2 1.3 1.4 2 3.1 2 5.1 0 1.6-.6 3.1-1.6 4.4-1 1.2-2.4 2.1-4 2.4-1.6.3-3.2.1-4.6-.7-1.4-.8-2.5-2-3.1-3.5C.9 9.2.8 7.5 1.3 6c.5-1.6 1.4-2.9 2.8-3.8C5.4 1.3 7 .9 8.6 1zm.5 12.9c1.3-.3 2.5-1 3.4-2.1.8-1.1 1.3-2.4 1.2-3.8 0-1.6-.6-3.2-1.7-4.3-1-1-2.2-1.6-3.6-1.7-1.3-.1-2.7.2-3.8 1-1.1.8-1.9 1.9-2.3 3.3-.4 1.3-.4 2.7.2 4 .6 1.3 1.5 2.3 2.7 3 1.2.7 2.6.9 3.9.6z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M6 5l.777-.416 4.5 3v.832l-4.5 3L6 11V5zm1 .934v4.132L10.099 8 7 5.934z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPlay;
impl IconShape for VsPlay {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.78 2L3 2.41v12l.78.42 9-6V8l-9-6zM4 13.48V3.35l7.6 5.07L4 13.48z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPlug;
impl IconShape for VsPlug {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7 1H6v3H4.5l-.5.5V8a4 4 0 0 0 3.5 3.969V15h1v-3.031A4 4 0 0 0 12 8V4.5l-.5-.5H10V1H9v3H7V1zm3.121 9.121A3 3 0 0 1 5 8V5h6v3a3 3 0 0 1-.879 2.121z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPreserveCase;
impl IconShape for VsPreserveCase {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.51358 11H7.51456L6.69815 8.84082H3.43253L2.66446 11H1.66006L4.61417 3.29785H5.54874L8.51358 11ZM6.40274 8.02979L5.19424 4.74805C5.15486 4.64062 5.11547 4.46875 5.07608 4.23242H5.0546C5.01879 4.45085 4.97761 4.62272 4.93106 4.74805L3.73331 8.02979H6.40274Z",
            }
            path {
                d: "M9.59725 11V3.29785H11.7887C12.4547 3.29785 12.9828 3.46077 13.3731 3.78662C13.7634 4.11247 13.9586 4.53678 13.9586 5.05957C13.9586 5.49642 13.8404 5.87598 13.6041 6.19824C13.3678 6.52051 13.0419 6.74967 12.6265 6.88574V6.90723C13.1458 6.9681 13.5611 7.16504 13.8726 7.49805C14.1842 7.82747 14.3399 8.25716 14.3399 8.78711C14.3399 9.44596 14.1036 9.97949 13.6309 10.3877C13.1583 10.7959 12.5621 11 11.8424 11H9.59725ZM10.4996 4.11426V6.60107H11.4234C11.9176 6.60107 12.3061 6.48291 12.589 6.24658C12.8718 6.00667 13.0133 5.67008 13.0133 5.23682C13.0133 4.48844 12.5209 4.11426 11.5362 4.11426H10.4996ZM10.4996 7.41211V10.1836H11.7242C12.2542 10.1836 12.6641 10.0583 12.9542 9.80762C13.2478 9.55697 13.3946 9.21322 13.3946 8.77637C13.3946 7.86686 12.7751 7.41211 11.5362 7.41211H10.4996Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPreview;
impl IconShape for VsPreview {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 2h12l1 1v10l-1 1H2l-1-1V3l1-1zm0 11h12V3H2v10zm11-9H3v3h10V4zm-1 2H4V5h8v1zm-3 6h4V8H9v4zm1-3h2v2h-2V9zM7 8H3v1h4V8zm-4 3h4v1H3v-1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPrimitiveSquare;
impl IconShape for VsPrimitiveSquare {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3.5 4l.5-.5h8l.5.5v8l-.5.5H4l-.5-.5V4zm1 .5v7h7v-7h-7z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsProject;
impl IconShape for VsProject {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 1h13l.5.5v13l-.5.5h-13l-.5-.5v-13l.5-.5zM2 14h12V2H2v12zM3 3h2v10H3V3zm6 0H7v6h2V3zm2 0h2v8h-2V3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPulse;
impl IconShape for VsPulse {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.8 9L10 3H9L7.158 9.64 5.99 4.69h-.97L3.85 9H1v.99h3.23l.49-.37.74-2.7L6.59 12h1.03l1.87-7.04 1.46 4.68.48.36H15V9h-3.2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsPython;
impl IconShape for VsPython {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.55112,11.4558c-0.67083,-0.2348 -1.14116,-0.9302 -1.42492,-2.10673c-0.11324,-0.46969 -0.15128,-1.03614 -0.11004,-1.64126c0.06047,-0.88847 0.17775,-1.39694 0.45247,-1.96183c0.1468,-0.30184 0.20649,-0.38829 0.4152,-0.60152c0.26344,-0.26906 0.53487,-0.42755 0.86458,-0.50499c0.12461,-0.02881 0.81498,-0.04322 2.74073,-0.05643l2.57416,-0.01801v-0.39861l-3.36976,-0.02401l-0.01826,-0.06843c-0.02929,-0.11046 -0.02063,-1.17734 0.01198,-1.49191c0.0498,-0.47857 0.14833,-0.7325 0.38275,-0.98632c0.16827,-0.18213 0.52996,-0.36715 0.88842,-0.45432c0.51644,-0.1257 0.74977,-0.14143 2.08961,-0.14143c1.35513,0 1.69787,0.02401 2.08866,0.14348c0.5341,0.164 1.0805,0.66731 1.2427,1.1448c0.105,0.3088 0.1211,0.62445 0.1206,2.36008c-0.0005,1.83961 -0.0071,1.91981 -0.1745,2.23641c-0.1776,0.33522 -0.5099,0.47576 -0.9173,0.6298c-0.1695,0.06364 -0.2266,0.04193 -1.9707,0.04793c-1.87968,0.01201 -3.44738,-0.0085 -3.8613,0.13932c-0.47221,0.14852 -0.64831,0.3104 -0.85741,0.73915c-0.19911,0.40809 -0.0794,1.09686 -0.09379,2.18403v0.4375v0.4375c-0.80996,0.0288 -0.90396,0.0068 -1.07395,-0.052zM6.49078,3.1591c0.20163,-0.13616 0.27024,-0.27039 0.27024,-0.52852c0,-0.19775 -0.00948,-0.23641 -0.09224,-0.35491c-0.18912,-0.27266 -0.53021,-0.35731 -0.82383,-0.20435c-0.46446,0.24205 -0.44249,0.90696 0.03746,1.13544c0.17274,0.08164 0.44783,0.06159 0.60837,-0.04766z",
            }
            path {
                d: "M13.4489,4.54418c0.6708,0.23485 1.1411,0.93025 1.4249,2.10675c0.1132,0.46969 0.1513,1.03615 0.11,1.64126c-0.0604,0.88847 -0.1777,1.39694 -0.4524,1.96181c-0.1468,0.3019 -0.2065,0.3883 -0.4152,0.6015c-0.2635,0.2691 -0.5349,0.4276 -0.8646,0.505c-0.1246,0.0288 -0.815,0.0432 -2.7407,0.0565l-2.5742,0.018v0.3986l3.3698,0.024l0.0182,0.0684c0.0293,0.1105 0.0207,1.1774 -0.012,1.4919c-0.0498,0.4786 -0.1483,0.7325 -0.3827,0.9863c-0.1683,0.1822 -0.53,0.3672 -0.8884,0.4544c-0.51646,0.1257 -0.7498,0.1414 -2.08964,0.1414c-1.35513,0 -1.69787,-0.024 -2.08862,-0.1435c-0.53409,-0.164 -1.08054,-0.6673 -1.24277,-1.1448c-0.10493,-0.3088 -0.12102,-0.6244 -0.12056,-2.3601c0.00049,-1.83957 -0.03841,-2.07089 0.12897,-2.3875c0.1776,-0.33522 0.20721,-0.31874 0.61461,-0.47278c0.16957,-0.06363 0.57482,-0.04782 2.31891,-0.05382c1.87968,-0.01201 3.1327,0.02273 3.4504,-0.0768c0.4722,-0.14852 0.9773,-0.35914 1.1864,-0.78788c0.1991,-0.4081 0.1613,-0.67315 0.1757,-1.76032v-0.875v-0.4375c0.81,-0.02881 0.904,-0.00684 1.074,0.05199zM9.50922,12.8409c-0.20163,0.1362 -0.27024,0.2704 -0.27024,0.5285c0,0.1978 0.00948,0.2364 0.09224,0.3549c0.18912,0.2727 0.53021,0.3573 0.82388,0.2044c0.4644,-0.2421 0.4424,-0.907 -0.0375,-1.1355c-0.17275,-0.0816 -0.44784,-0.0616 -0.60838,0.0477z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsQuestion;
impl IconShape for VsQuestion {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.5 1a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13zm0 12a5.5 5.5 0 1 1 0-11 5.5 5.5 0 0 1 0 11zm1.55-8.42a1.84 1.84 0 0 0-.61-.42A2.25 2.25 0 0 0 7.53 4a2.16 2.16 0 0 0-.88.17c-.239.1-.45.254-.62.45a1.89 1.89 0 0 0-.38.62 3 3 0 0 0-.15.72h1.23a.84.84 0 0 1 .506-.741.72.72 0 0 1 .304-.049.86.86 0 0 1 .27 0 .64.64 0 0 1 .22.14.6.6 0 0 1 .16.22.73.73 0 0 1 .06.3c0 .173-.037.343-.11.5a2.4 2.4 0 0 1-.27.46l-.35.42c-.12.13-.24.27-.35.41a2.33 2.33 0 0 0-.27.45 1.18 1.18 0 0 0-.1.5v.66H8v-.49a.94.94 0 0 1 .11-.42 3.09 3.09 0 0 1 .28-.41l.36-.44a4.29 4.29 0 0 0 .36-.48 2.59 2.59 0 0 0 .28-.55 1.91 1.91 0 0 0 .11-.64 2.18 2.18 0 0 0-.1-.67 1.52 1.52 0 0 0-.35-.55zM6.8 9.83h1.17V11H6.8V9.83z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsQuote;
impl IconShape for VsQuote {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.16 3.5C4.73 5.06 3.55 6.67 3.55 9.36c.16-.05.3-.05.44-.05 1.27 0 2.5.86 2.5 2.41 0 1.61-1.03 2.61-2.5 2.61-1.9 0-2.99-1.52-2.99-4.25 0-3.8 1.75-6.53 5.02-8.42L7.16 3.5zm7 0c-2.43 1.56-3.61 3.17-3.61 5.86.16-.05.3-.05.44-.05 1.27 0 2.5.86 2.5 2.41 0 1.61-1.03 2.61-2.5 2.61-1.89 0-2.98-1.52-2.98-4.25 0-3.8 1.75-6.53 5.02-8.42l1.14 1.84h-.01z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRadioTower;
impl IconShape for VsRadioTower {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.998 5.58a5.55 5.55 0 0 1 1.62-3.88l-.71-.7a6.45 6.45 0 0 0 0 9.16l.71-.7a5.55 5.55 0 0 1-1.62-3.88zm1.06 0a4.42 4.42 0 0 0 1.32 3.17l.71-.71a3.27 3.27 0 0 1-.76-1.12 3.45 3.45 0 0 1 0-2.67 3.22 3.22 0 0 1 .76-1.13l-.71-.71a4.46 4.46 0 0 0-1.32 3.17zm7.65 3.21l-.71-.71c.33-.32.59-.704.76-1.13a3.449 3.449 0 0 0 0-2.67 3.22 3.22 0 0 0-.76-1.13l.71-.7a4.468 4.468 0 0 1 0 6.34zM13.068 1l-.71.71a5.43 5.43 0 0 1 0 7.74l.71.71a6.45 6.45 0 0 0 0-9.16zM9.993 5.43a1.5 1.5 0 0 1-.245.98 2 2 0 0 1-.27.23l3.44 7.73-.92.4-.77-1.73h-5.54l-.77 1.73-.92-.4 3.44-7.73a1.52 1.52 0 0 1-.33-1.63 1.55 1.55 0 0 1 .56-.68 1.5 1.5 0 0 1 2.325 1.1zm-1.595-.34a.52.52 0 0 0-.25.14.52.52 0 0 0-.11.22.48.48 0 0 0 0 .29c.04.09.102.17.18.23a.54.54 0 0 0 .28.08.51.51 0 0 0 .5-.5.54.54 0 0 0-.08-.28.58.58 0 0 0-.23-.18.48.48 0 0 0-.29 0zm.23 2.05h-.27l-.87 1.94h2l-.86-1.94zm2.2 4.94l-.89-2h-2.88l-.89 2h4.66z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsReactions;
impl IconShape for VsReactions {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12 7.5c0 .169-.01.336-.027.5h1.005A5.5 5.5 0 1 0 8 12.978v-1.005A4.5 4.5 0 1 1 12 7.5zM5.5 7a1 1 0 1 0 0-2 1 1 0 0 0 0 2zm2 2.5c.712 0 1.355-.298 1.81-.776l.707.708A3.49 3.49 0 0 1 7.5 10.5a3.49 3.49 0 0 1-2.555-1.108l.707-.708A2.494 2.494 0 0 0 7.5 9.5zm2-2.5a1 1 0 1 0 0-2 1 1 0 0 0 0 2zm2.5 3h1v2h2v1h-2v2h-1v-2h-2v-1h2v-2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRecordKeys;
impl IconShape for VsRecordKeys {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14 3H3a1 1 0 0 0-1 1v7a1 1 0 0 0 1 1h11a1 1 0 0 0 1-1V4a1 1 0 0 0-1-1zm0 8H3V4h11v7zm-3-6h-1v1h1V5zm-1 2H9v1h1V7zm2-2h1v1h-1V5zm1 4h-1v1h1V9zM6 9h5v1H6V9zm7-2h-2v1h2V7zM8 5h1v1H8V5zm0 2H7v1h1V7zM4 9h1v1H4V9zm0-4h1v1H4V5zm3 0H6v1h1V5zM4 7h2v1H4V7z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRecordSmall;
impl IconShape for VsRecordSmall {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.00024 9C8.55253 9 9.00024 8.55228 9.00024 8C9.00024 7.44772 8.55253 7 8.00024 7C7.44796 7 7.00024 7.44772 7.00024 8C7.00024 8.55228 7.44796 9 8.00024 9Z",
            }
            path {
                d: "M12.0002 8C12.0002 10.2091 10.2094 12 8.00024 12C5.79111 12 4.00024 10.2091 4.00024 8C4.00024 5.79086 5.79111 4 8.00024 4C10.2094 4 12.0002 5.79086 12.0002 8ZM11.0002 8C11.0002 6.34315 9.6571 5 8.00024 5C6.34339 5 5.00024 6.34315 5.00024 8C5.00024 9.65685 6.34339 11 8.00024 11C9.6571 11 11.0002 9.65685 11.0002 8Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRecord;
impl IconShape for VsRecord {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 10a2 2 0 1 0 0-4 2 2 0 0 0 0 4z",
            }
            path {
                clip_rule: "evenodd",
                d: "M8.6 1c1.6.1 3.1.9 4.2 2 1.3 1.4 2 3.1 2 5.1 0 1.6-.6 3.1-1.6 4.4-1 1.2-2.4 2.1-4 2.4-1.6.3-3.2.1-4.6-.7-1.4-.8-2.5-2-3.1-3.5C.9 9.2.8 7.5 1.3 6c.5-1.6 1.4-2.9 2.8-3.8C5.4 1.3 7 .9 8.6 1zm.5 12.9c1.3-.3 2.5-1 3.4-2.1.8-1.1 1.3-2.4 1.2-3.8 0-1.6-.6-3.2-1.7-4.3-1-1-2.2-1.6-3.6-1.7-1.3-.1-2.7.2-3.8 1-1.1.8-1.9 1.9-2.3 3.3-.4 1.3-.4 2.7.2 4 .6 1.3 1.5 2.3 2.7 3 1.2.7 2.6.9 3.9.6z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRedo;
impl IconShape for VsRedo {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.5 2v3.5L12 6H8.5V5h2.521l-.941-.941a3.552 3.552 0 1 0-5.023 5.023l5.197 5.198-.72.72-5.198-5.198A4.57 4.57 0 0 1 10.8 3.339l.7.7V2h1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsReferences;
impl IconShape for VsReferences {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.105 4.561l-3.43 3.427-1.134-1.12 2.07-2.08h-4.8a2.4 2.4 0 1 0 0 4.8h.89v1.6h-.88a4 4 0 0 1 0-7.991h4.8L6.54 1.13 7.675 0l3.43 3.432v1.13zM16.62 24h-9.6l-.8-.8V10.412l.8-.8h9.6l.8.8V23.2l-.8.8zm-8.8-1.6h8V11.212h-8V22.4zm5.6-20.798h9.6l.8.8v12.786l-.8.8h-4v-1.6h3.2V3.2h-8v4.787h-1.6V2.401l.8-.8zm.8 11.186h-4.8v1.6h4.8v-1.6zm-4.8 3.2h4.8v1.6h-4.8v-1.6zm4.8 3.2h-4.8v1.6h4.8v-1.6zm1.6-14.4h4.8v1.6h-4.8v-1.6zm4.8 6.4h-1.6v1.6h1.6v-1.6zm-3.338-3.176v-.024h3.338v1.6h-1.762l-1.576-1.576z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRefresh;
impl IconShape for VsRefresh {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4.681 3H2V2h3.5l.5.5V6H5V4a5 5 0 1 0 4.53-.761l.302-.954A6 6 0 1 1 4.681 3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRegex;
impl IconShape for VsRegex {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.012 2h.976v3.113l2.56-1.557.486.885L11.47 6l2.564 1.559-.485.885-2.561-1.557V10h-.976V6.887l-2.56 1.557-.486-.885L9.53 6 6.966 4.441l.485-.885 2.561 1.557V2zM2 10h4v4H2v-4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRemoteExplorer;
impl IconShape for VsRemoteExplorer {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.344 2.125h20.312l.782.781v8.599a7.825 7.825 0 0 0-1.563-.912V3.688H2.125V17.75h7.813a7.813 7.813 0 0 0 1.562 4.688H5.25v-1.563h4.688v-1.563H1.344l-.782-.78V2.905l.782-.781zM17.75 11.5a6.25 6.25 0 1 0 0 12.5 6.25 6.25 0 0 0 0-12.5zm0 10.938a4.688 4.688 0 1 1 0-9.377 4.688 4.688 0 0 1 0 9.377zm2.603-3.132L18.2 17.153 20.353 15l.647.646-1.506 1.507L21 18.659l-.647.647zM15 17.246l1.506 1.507L15 20.259l.647.647 2.153-2.153-2.153-2.153-.647.646z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRemote;
impl IconShape for VsRemote {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.904 9.57L8.928 5.596l3.976-3.976-.619-.62L8 5.286v.619l4.285 4.285.62-.618zM3 5.62l4.072 4.07L3 13.763l.619.618L8 10v-.619L3.619 5 3 5.619z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRemove;
impl IconShape for VsRemove {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 8H1V7h14v1z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsReplaceAll;
impl IconShape for VsReplaceAll {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.6 2.677c.147-.31.356-.465.626-.465.248 0 .44.118.573.353.134.236.201.557.201.966 0 .443-.078.798-.235 1.067-.156.268-.365.402-.627.402-.237 0-.416-.125-.537-.374h-.008v.31H11V1h.593v1.677h.008zm-.016 1.1a.78.78 0 0 0 .107.426c.071.113.163.169.274.169.136 0 .24-.072.314-.216.075-.145.113-.35.113-.615 0-.22-.035-.39-.104-.514-.067-.124-.164-.187-.29-.187-.12 0-.219.062-.297.185a.886.886 0 0 0-.117.48v.272zM4.12 7.695L2 5.568l.662-.662 1.006 1v-1.51A1.39 1.39 0 0 1 5.055 3H7.4v.905H5.055a.49.49 0 0 0-.468.493l.007 1.5.949-.944.656.656-2.08 2.085zM9.356 4.93H10V3.22C10 2.408 9.685 2 9.056 2c-.135 0-.285.024-.45.073a1.444 1.444 0 0 0-.388.167v.665c.237-.203.487-.304.75-.304.261 0 .392.156.392.469l-.6.103c-.506.086-.76.406-.76.961 0 .263.061.473.183.631A.61.61 0 0 0 8.69 5c.29 0 .509-.16.657-.48h.009v.41zm.004-1.355v.193a.75.75 0 0 1-.12.436.368.368 0 0 1-.313.17.276.276 0 0 1-.22-.095.38.38 0 0 1-.08-.248c0-.222.11-.351.332-.389l.4-.067zM7 12.93h-.644v-.41h-.009c-.148.32-.367.48-.657.48a.61.61 0 0 1-.507-.235c-.122-.158-.183-.368-.183-.63 0-.556.254-.876.76-.962l.6-.103c0-.313-.13-.47-.392-.47-.263 0-.513.102-.75.305v-.665c.095-.063.224-.119.388-.167.165-.049.315-.073.45-.073.63 0 .944.407.944 1.22v1.71zm-.64-1.162v-.193l-.4.068c-.222.037-.333.166-.333.388 0 .1.027.183.08.248a.276.276 0 0 0 .22.095.368.368 0 0 0 .312-.17c.08-.116.12-.26.12-.436zM9.262 13c.321 0 .568-.058.738-.173v-.71a.9.9 0 0 1-.552.207.619.619 0 0 1-.5-.215c-.12-.145-.181-.345-.181-.598 0-.26.063-.464.189-.612a.644.644 0 0 1 .516-.223c.194 0 .37.069.528.207v-.749c-.129-.09-.338-.134-.626-.134-.417 0-.751.14-1.001.422-.249.28-.373.662-.373 1.148 0 .42.116.764.349 1.03.232.267.537.4.913.4zM2 9l1-1h9l1 1v5l-1 1H3l-1-1V9zm1 0v5h9V9H3zm3-2l1-1h7l1 1v5l-1 1V7H6z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsReplace;
impl IconShape for VsReplace {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3.221 3.739l2.261 2.269L7.7 3.784l-.7-.7-1.012 1.007-.008-1.6a.523.523 0 0 1 .5-.526H8V1H6.48A1.482 1.482 0 0 0 5 2.489V4.1L3.927 3.033l-.706.706zm6.67 1.794h.01c.183.311.451.467.806.467.393 0 .706-.168.94-.503.236-.335.353-.78.353-1.333 0-.511-.1-.913-.301-1.207-.201-.295-.488-.442-.86-.442-.405 0-.718.194-.938.581h-.01V1H9v4.919h.89v-.386zm-.015-1.061v-.34c0-.248.058-.448.175-.601a.54.54 0 0 1 .445-.23.49.49 0 0 1 .436.233c.104.154.155.368.155.643 0 .33-.056.587-.169.768a.524.524 0 0 1-.47.27.495.495 0 0 1-.411-.211.853.853 0 0 1-.16-.532zM9 12.769c-.256.154-.625.231-1.108.231-.563 0-1.02-.178-1.369-.533-.349-.355-.523-.813-.523-1.374 0-.648.186-1.158.56-1.53.374-.376.875-.563 1.5-.563.433 0 .746.06.94.179v.998a1.26 1.26 0 0 0-.792-.276c-.325 0-.583.1-.774.298-.19.196-.283.468-.283.816 0 .338.09.603.272.797.182.191.431.287.749.287.282 0 .558-.092.828-.276v.946zM4 7L3 8v6l1 1h7l1-1V8l-1-1H4zm0 1h7v6H4V8z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsReply;
impl IconShape for VsReply {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6.306 2.146l-4.02 4.02v.708l4.02 4.02.708-.707L3.807 6.98H5.69c2.813 0 4.605.605 5.705 1.729 1.102 1.125 1.615 2.877 1.615 5.421v.35h1v-.35c0-2.646-.527-4.72-1.9-6.121C10.735 6.605 8.617 5.98 5.69 5.98H3.887l3.127-3.126-.708-.708z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRepoClone;
impl IconShape for VsRepoClone {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13 10H4V2h4V1H3.74a1.9 1.9 0 0 0-.67.13 1.66 1.66 0 0 0-.57.41 1.73 1.73 0 0 0-.37.59 1.68 1.68 0 0 0-.13.62v9.5a1.75 1.75 0 0 0 1.07 1.62 1.9 1.9 0 0 0 .67.13H4v-1h-.26a.72.72 0 0 1-.29-.06.78.78 0 0 1-.4-.4.93.93 0 0 1 0-.29v-.5a.93.93 0 0 1 0-.29.78.78 0 0 1 .4-.4.72.72 0 0 1 .29-.06H13v2H9v1h4.5l.5-.5V9h-1v1zM6 3H5v1h1V3zM5 5h1v1H5V5zm0 2h1v1H5V7zm.28 8H5v-3h3v3h-.28L6.5 13.49 5.28 15zM10 1h4.5l.5.5v6l-.5.5H12v1h-1V8h-1a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1zm.5 6h.5V6h-.5a.5.5 0 0 0 0 1zM12 7h2V6h-2v1zm-1-2h3V2h-3v3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRepoFetch;
impl IconShape for VsRepoFetch {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "1",
                rx: "0.5",
                width: "1",
                x: "8",
                y: "4",
            }
            rect {
                height: "1",
                rx: "0.5",
                width: "1",
                x: "8",
                y: "6",
            }
            rect {
                height: "1",
                rx: "0.5",
                width: "1",
                x: "8",
                y: "2",
            }
            rect {
                height: "1",
                rx: "0.5",
                width: "1",
                x: "8",
            }
            path {
                clip_rule: "evenodd",
                d: "M8.00016 6.43935L9.00016 6.43935L12.6466 2.79291L13.3537 3.50001L8.85371 8.00001H8.14661L3.64661 3.50001L4.35371 2.79291L8.00016 6.43935Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M5.03544 12H2V13H5.03544C5.27806 14.6961 6.73676 16 8.5 16C10.2632 16 11.7219 14.6961 11.9646 13H15.0001V12H11.9646C11.7219 10.3039 10.2632 9 8.5 9C6.73676 9 5.27806 10.3039 5.03544 12ZM11 12.5C11 13.8807 9.88071 15 8.5 15C7.11929 15 6 13.8807 6 12.5C6 11.1193 7.11929 10 8.5 10C9.88071 10 11 11.1193 11 12.5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRepoForcePush;
impl IconShape for VsRepoForcePush {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.00016 2.56065L8 3.5H9L9.00016 2.56065L12.6466 6.20709L13.3537 5.49999L8.85371 0.999986H8.14661L3.64661 5.49999L4.35371 6.20709L8.00016 2.56065Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M8.00016 5.56065V7.99999H9.00016V5.56065L12.6466 9.20709L13.3537 8.49999L8.85371 3.99999H8.14661L3.64661 8.49999L4.35371 9.20709L8.00016 5.56065Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M5.03544 12H2V13H5.03544C5.27806 14.6961 6.73676 16 8.5 16C10.2632 16 11.7219 14.6961 11.9646 13H15.0001V12H11.9646C11.7219 10.3039 10.2632 9 8.5 9C6.73676 9 5.27806 10.3039 5.03544 12ZM11 12.5C11 13.8807 9.88071 15 8.5 15C7.11929 15 6 13.8807 6 12.5C6 11.1193 7.11929 10 8.5 10C9.88071 10 11 11.1193 11 12.5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRepoForked;
impl IconShape for VsRepoForked {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 4a2 2 0 1 0-2.47 1.94V7a.48.48 0 0 1-.27.44L8.49 8.88l-2.76-1.4A.49.49 0 0 1 5.46 7V5.94a2 2 0 1 0-1 0V7a1.51 1.51 0 0 0 .82 1.34L8 9.74v1.32a2 2 0 1 0 1 0V9.74l2.7-1.36A1.49 1.49 0 0 0 12.52 7V5.92A2 2 0 0 0 14 4zM4 4a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm5.47 9a1 1 0 1 1-2 0 1 1 0 0 1 2 0zM12 5a1 1 0 1 1 0-2 1 1 0 0 1 0 2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRepoPull;
impl IconShape for VsRepoPull {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.03544 12H2V13H5.03544C5.27806 14.6961 6.73676 16 8.5 16C10.2632 16 11.7219 14.6961 11.9646 13H15.0001V12H11.9646C11.7219 10.3039 10.2632 9 8.5 9C6.73676 9 5.27806 10.3039 5.03544 12ZM11 12.5C11 13.8807 9.88071 15 8.5 15C7.11929 15 6 13.8807 6 12.5C6 11.1193 7.11929 10 8.5 10C9.88071 10 11 11.1193 11 12.5Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M8.00016 6.43934V0H9.00016V6.43934L12.6466 2.7929L13.3537 3.5L8.85371 8H8.14661L3.64661 3.5L4.35371 2.7929L8.00016 6.43934Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRepoPush;
impl IconShape for VsRepoPush {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.00016 2.56066V8H9.00016V2.56066L12.6466 6.2071L13.3537 5.5L8.85371 0.999996H8.14661L3.64661 5.5L4.35371 6.2071L8.00016 2.56066Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M5.03544 12H2V13H5.03544C5.27806 14.6961 6.73676 16 8.5 16C10.2632 16 11.7219 14.6961 11.9646 13H15.0001V12H11.9646C11.7219 10.3039 10.2632 9 8.5 9C6.73676 9 5.27806 10.3039 5.03544 12ZM11 12.5C11 13.8807 9.88071 15 8.5 15C7.11929 15 6 13.8807 6 12.5C6 11.1193 7.11929 10 8.5 10C9.88071 10 11 11.1193 11 12.5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRepo;
impl IconShape for VsRepo {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14 10V1.5l-.5-.5H3.74a1.9 1.9 0 0 0-.67.13 1.77 1.77 0 0 0-.94 1 1.7 1.7 0 0 0-.13.62v9.5a1.7 1.7 0 0 0 .13.67c.177.427.515.768.94.95a1.9 1.9 0 0 0 .67.13H4v-1h-.26a.72.72 0 0 1-.29-.06.74.74 0 0 1-.4-.4.93.93 0 0 1-.05-.29v-.5a.93.93 0 0 1 .05-.29.74.74 0 0 1 .4-.4.72.72 0 0 1 .286-.06H13v2H9v1h4.5l.5-.5V10zM4 10V2h9v8H4zm1-7h1v1H5V3zm0 2h1v1H5V5zm1 2H5v1h1V7zm.5 6.49L5.28 15H5v-3h3v3h-.28L6.5 13.49z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsReport;
impl IconShape for VsReport {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 1h13l.5.5v10l-.5.5H7.707l-2.853 2.854L4 14.5V12H1.5l-.5-.5v-10l.5-.5zm6 10H14V2H2v9h2.5l.5.5v1.793l2.146-2.147L7.5 11zm0-8h1v5h-1V3zm0 7h1V9h-1v1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRequestChanges;
impl IconShape for VsRequestChanges {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.7099 1.29L13.7099 4.29L13.9999 5V14L12.9999 15H3.99994L2.99994 14V2L3.99994 1H9.99994L10.7099 1.29ZM3.99994 14H12.9999V5L9.99994 2H3.99994V14ZM8 6H6V7H8V9H9V7H11V6H9V4H8V6ZM6 11H11V12H6V11Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRobot;
impl IconShape for VsRobot {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8 8C8 9.10457 7.10457 10 6 10C4.89543 10 4 9.10457 4 8C4 6.89543 4.89543 6 6 6C7.10457 6 8 6.89543 8 8ZM5 8C5 8.55228 5.44772 9 6 9C6.55228 9 7 8.55228 7 8C7 7.44772 6.55228 7 6 7C5.44772 7 5 7.44772 5 8Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M8 8C8 9.10457 8.89543 10 10 10C11.1046 10 12 9.10457 12 8C12 6.89543 11.1046 6 10 6C8.89543 6 8 6.89543 8 8ZM11 8C11 8.55228 10.5523 9 10 9C9.44772 9 9 8.55228 9 8C9 7.44772 9.44772 7 10 7C10.5523 7 11 7.44772 11 8Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M8.51001 11.251C9.02924 11.1436 9.50557 10.8864 9.88001 10.511L10.58 11.221C9.89283 11.901 8.96678 12.2851 8.00001 12.291C7.51235 12.2873 7.03006 12.1888 6.58001 12.001C6.13507 11.8188 5.73061 11.5503 5.39001 11.211L6.09001 10.501C6.40274 10.8119 6.78661 11.0418 7.20833 11.1708C7.63005 11.2998 8.07687 11.3238 8.51001 11.241V11.251Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M9.5 1.5C9.5 2.15311 9.0826 2.70873 8.5 2.91465V3H11C12.6569 3 14 4.34315 14 6V7L15 8V10L14 11V12C14 13.6569 12.6569 15 11 15H5C3.34315 15 2 13.6569 2 12V11L1 10V8L2 7V6C2 4.34315 3.34315 3 5 3H7.5V2.91465C6.9174 2.70873 6.5 2.15311 6.5 1.5C6.5 0.671573 7.17157 0 8 0C8.82843 0 9.5 0.671573 9.5 1.5ZM5 4C3.89543 4 3 4.89543 3 6V12C3 13.1046 3.89543 14 5 14H11C12.1046 14 13 13.1046 13 12V6C13 4.89543 12.1046 4 11 4H5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRocket;
impl IconShape for VsRocket {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.491 1c-3.598.004-6.654 1.983-8.835 4H1.5l-.5.5v3l.147.354.991.991.001.009 4 4 .009.001.999.999L7.5 15h3l.5-.5v-4.154c2.019-2.178 3.996-5.233 3.992-8.846l-.501-.5zM2 6h2.643a23.828 23.828 0 0 0-2.225 2.71L2 8.294V6zm5.7 8l-.42-.423a23.59 23.59 0 0 0 2.715-2.216V14H7.7zm-1.143-1.144L3.136 9.437C4.128 8 8.379 2.355 13.978 2.016c-.326 5.612-5.987 9.853-7.421 10.84zM4 15v-1H2v-2H1v3h3zm6.748-7.667a1.5 1.5 0 1 0-2.496-1.666 1.5 1.5 0 0 0 2.495 1.666z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRootFolderOpened;
impl IconShape for VsRootFolderOpened {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1 6.257V2.5l.5-.5h5l.35.15.86.85h5.79l.5.5V6h1.13l.48.63-2.63 7-.48.37H8.743a5.48 5.48 0 0 0 .657-1h2.73l2.37-6H8.743a5.534 5.534 0 0 0-.72-.724l.127-.126L8.5 6H13V4H7.5l-.35-.15L6.29 3H2l.01 2.594c-.361.184-.7.407-1.01.663z",
                fill_rule: "evenodd",
            }
            path {
                d: "M6 10.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0z",
            }
            path {
                clip_rule: "evenodd",
                d: "M8 10.5a3.5 3.5 0 1 1-7 0 3.5 3.5 0 0 1 7 0zM4.5 13a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRootFolder;
impl IconShape for VsRootFolder {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.71 3h6.79l.51.5v10l-.5.5H8.743a5.48 5.48 0 0 0 .657-1h4.59v-1.51l.01-4v-1.5H7.69l-.017.017a5.494 5.494 0 0 0-.881-.508l.348-.349.35-.15h6.5l.01-.99H7.5l-.36-.15-.85-.85H2V5.6a5.45 5.45 0 0 0-.99.649V2.5l.5-.5h5l.35.15.85.85z",
                fill_rule: "evenodd",
            }
            path {
                d: "M6 10.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0z",
            }
            path {
                clip_rule: "evenodd",
                d: "M8 10.5a3.5 3.5 0 1 1-7 0 3.5 3.5 0 0 1 7 0zM4.5 13a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRss;
impl IconShape for VsRss {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 13H3v-2c1.11 0 2 .89 2 2zM3 3v1a9 9 0 0 1 9 9h1C13 7.48 8.52 3 3 3zm0 4v1c2.75 0 5 2.25 5 5h1c0-3.31-2.69-6-6-6z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRuby;
impl IconShape for VsRuby {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1 7.19l6.64 6.64h.72L15 7.19v-.72l-3.32-3.32-.36-.15H4.68l-.36.15L1 6.47v.72zm7 5.56L2.08 6.83 4.89 4h6.22l2.81 2.83L8 12.75zm0-7.73h2.69l1.81 1.81-4.5 4.4V5.02z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRunAbove;
impl IconShape for VsRunAbove {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.77 1.01L1 1.42v12l.78.42 9-6v-.83l-9.01-6zM2 12.49V2.36l7.6 5.07L2 12.49zM12.15 8h.71l2.5 2.5-.71.71L13 9.56V15h-1V9.55l-1.65 1.65-.7-.7 2.5-2.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRunAllCoverage;
impl IconShape for VsRunAllCoverage {
    fn view_box(&self) -> &str {
        "0 0 17 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 13.3497L15.7795 8.83V8L6.99951 2.14667V3.35L14.5995 8.42L9 12.1481V13.3497Z",
            }
            path {
                d: "M2.99951 2.41L3.77951 2L12.7795 8V8.83L9 11.3497V10.1507L11.5995 8.42L3.99951 3.35V7H2.99951V2.41Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M4.87227 7.80803C4.02215 7.7549 3.16715 7.9667 2.46857 8.44931C1.71646 8.9338 1.23555 9.6327 0.970474 10.4798C0.70131 11.2888 0.756984 12.1983 1.07646 12.997C1.39862 13.8024 1.98841 14.444 2.73373 14.8699C3.48976 15.3019 4.34985 15.407 5.20068 15.2475C6.06198 15.086 6.81126 14.6028 7.34443 13.963L7.34919 13.9568C7.87759 13.2698 8.20141 12.468 8.20141 11.6053C8.20141 10.5403 7.82698 9.63047 7.13464 8.88488L7.12941 8.87965C6.54444 8.29468 5.74055 7.8623 4.87227 7.80803ZM2.95059 9.18281C3.4627 8.81037 4.12262 8.66604 4.74312 8.71377L4.74421 8.71385C5.40049 8.76073 5.9647 9.04069 6.44119 9.51719C6.95689 10.0329 7.24402 10.7907 7.24402 11.5546V11.5618L7.24453 11.5689C7.29089 12.2179 7.0608 12.8292 6.67758 13.3579C6.25159 13.8765 5.68456 14.2071 5.06818 14.3493C4.45885 14.49 3.80161 14.3963 3.23776 14.0674C2.66626 13.7341 2.23786 13.259 1.95045 12.6362C1.67052 12.0297 1.66791 11.3722 1.85872 10.752L1.85983 10.7482C2.04996 10.0827 2.42849 9.56252 2.95059 9.18281ZM6.35355 10.8536L4.35355 12.8536H3.64645L2.64645 11.8536L3.35355 11.1464L4 11.7929L5.64645 10.1464L6.35355 10.8536Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRunAll;
impl IconShape for VsRunAll {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.78 2L2 2.41v12l.78.42 9-6V8l-9-6zM3 13.48V3.35l7.6 5.07L3 13.48z",
            }
            path {
                clip_rule: "evenodd",
                d: "M6 14.683l8.78-5.853V8L6 2.147V3.35l7.6 5.07L6 13.48v1.203z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRunBelow;
impl IconShape for VsRunBelow {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.8 1.01l-.78.41v12l.78.42 9-6v-.83l-9-6zm.22 11.48V2.36l7.6 5.07-7.6 5.06zM12.85 15h-.71l-2.5-2.5.71-.71L12 13.44V8h1v5.45l1.65-1.65.71.71L12.85 15z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRunCoverage;
impl IconShape for VsRunCoverage {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 2.41L5.78 2L14.78 8V8.83L9 12.6833V11.4826L13.6 8.42L6 3.35V7H5V2.41Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M4.87227 7.80803C4.02215 7.7549 3.16715 7.9667 2.46857 8.44931C1.71646 8.9338 1.23555 9.6327 0.970474 10.4798C0.70131 11.2888 0.756984 12.1983 1.07646 12.997C1.39862 13.8024 1.98841 14.444 2.73373 14.8699C3.48976 15.3019 4.34985 15.407 5.20068 15.2475C6.06198 15.086 6.81126 14.6028 7.34443 13.963L7.34919 13.9568C7.87759 13.2698 8.20141 12.468 8.20141 11.6053C8.20141 10.5403 7.82698 9.63047 7.13464 8.88488L7.12941 8.87965C6.54444 8.29468 5.74055 7.8623 4.87227 7.80803ZM2.95059 9.18281C3.4627 8.81037 4.12262 8.66604 4.74312 8.71377L4.74421 8.71385C5.40049 8.76073 5.96469 9.04069 6.44119 9.51719C6.95689 10.0329 7.24402 10.7907 7.24402 11.5546V11.5618L7.24453 11.5689C7.29089 12.2179 7.0608 12.8292 6.67758 13.3579C6.25159 13.8765 5.68456 14.2071 5.06818 14.3493C4.45885 14.49 3.80161 14.3963 3.23776 14.0674C2.66626 13.7341 2.23786 13.259 1.95045 12.6362C1.67052 12.0297 1.66791 11.3722 1.85872 10.752L1.85983 10.7482C2.04996 10.0827 2.42849 9.56252 2.95059 9.18281ZM6.35355 10.8536L4.35355 12.8536H3.64645L2.64645 11.8536L3.35355 11.1464L4 11.7929L5.64645 10.1464L6.35355 10.8536Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsRunErrors;
impl IconShape for VsRunErrors {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 2.41L5.78 2L14.78 8V8.83L9 12.6833V11.4826L13.6 8.42L6 3.35V7H5V2.41Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M4.87227 7.80803C4.02215 7.7549 3.16715 7.9667 2.46857 8.44931C1.71646 8.9338 1.23555 9.6327 0.970474 10.4798C0.70131 11.2888 0.756984 12.1983 1.07646 12.997C1.39862 13.8024 1.98841 14.444 2.73373 14.8699C3.48976 15.3019 4.34985 15.407 5.20068 15.2475C6.06198 15.086 6.81126 14.6028 7.34443 13.963L7.34919 13.9568C7.87759 13.2698 8.20141 12.468 8.20141 11.6053C8.20141 10.5403 7.82698 9.63047 7.13464 8.88488L7.12941 8.87965C6.54444 8.29468 5.74055 7.8623 4.87227 7.80803ZM2.95059 9.18281C3.4627 8.81037 4.12262 8.66604 4.74312 8.71377L4.74421 8.71385C5.40049 8.76073 5.96469 9.04069 6.44119 9.51719C6.95689 10.0329 7.24402 10.7907 7.24402 11.5546V11.5618L7.24453 11.5689C7.29089 12.2179 7.0608 12.8292 6.67758 13.3579C6.25159 13.8765 5.68456 14.2071 5.06818 14.3493C4.45885 14.49 3.80161 14.3963 3.23776 14.0674C2.66626 13.7341 2.23786 13.259 1.95045 12.6362C1.67052 12.0297 1.66791 11.3722 1.85872 10.752L1.85983 10.7482C2.04996 10.0827 2.42849 9.56252 2.95059 9.18281ZM4.5051 11.0124L3.29191 9.7487L2.65431 10.3863L3.87316 11.6559L2.65431 12.9256L3.29191 13.5632L4.5051 12.2994L5.71829 13.5632L6.35589 12.9256L5.13704 11.6559L6.35589 10.3863L5.71829 9.7487L4.5051 11.0124Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSaveAll;
impl IconShape for VsSaveAll {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.85 2.65l-1.5-1.5L13 1H4.48l-.5.5V4H1.5l-.5.5v10l.5.5h10l.5-.5V12h2.5l.5-.5V3l-.15-.35zM11 14H2V5h1v3.07h6V5h.79L11 6.21V14zM6 7V5h2v2H6zm8 4h-2V6l-.15-.35-1.5-1.5L10 4H5V2h7.81l1.21 1.21L14 11z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSaveAs;
impl IconShape for VsSaveAs {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.04 1.33L12.71 3l.29.71v.33h-.5l-.5.5v-.83l-1.67-1.67H10v4H4v-4H2v10h3l-.5 1H2l-1-1v-10l1-1h8.33l.71.29zM7 5h2V2H7v3zm6.5 0L15 6.5l-.02.69-5.5 5.5-.13.12-.37.37-.1.09-3 1.5-.67-.67 1.5-3 .09-.1.37-.37.12-.13 5.5-5.5h.71zm-6.22 7.24l-.52 1 1.04-.48-.52-.52zm.69-1.03l.79.79 5.15-5.15-.79-.79-5.15 5.15z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSave;
impl IconShape for VsSave {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13.353 1.146l1.5 1.5L15 3v11.5l-.5.5h-13l-.5-.5v-13l.5-.5H13l.353.146zM2 2v12h12V3.208L12.793 2H11v4H4V2H2zm6 0v3h2V2H8z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsScreenFull;
impl IconShape for VsScreenFull {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 12h10V4H3v8zm2-6h6v4H5V6zM2 6H1V2.5l.5-.5H5v1H2v3zm13-3.5V6h-1V3h-3V2h3.5l.5.5zM14 10h1v3.5l-.5.5H11v-1h3v-3zM2 13h3v1H1.5l-.5-.5V10h1v3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsScreenNormal;
impl IconShape for VsScreenNormal {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.5 4H1V3h2V1h1v2.5l-.5.5zM13 3V1h-1v2.5l.5.5H15V3h-2zm-1 9.5V15h1v-2h2v-1h-2.5l-.5.5zM1 12v1h2v2h1v-2.5l-.5-.5H1zm11-1.5l-.5.5h-7l-.5-.5v-5l.5-.5h7l.5.5v5zM10 7H6v2h4V7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSearchFuzzy;
impl IconShape for VsSearchFuzzy {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 1.5C5.51472 1.5 3.5 3.51472 3.5 6C3.5 7.20114 3.9706 8.29237 4.73749 9.09937L1.04291 12.7939L1.75001 13.5011L5.5053 9.74577C6.2193 10.2222 7.07721 10.5 8 10.5C10.4853 10.5 12.5 8.48528 12.5 6C12.5 3.51472 10.4853 1.5 8 1.5ZM4.5 6C4.5 4.067 6.067 2.5 8 2.5C9.933 2.5 11.5 4.067 11.5 6C11.5 7.933 9.933 9.5 8 9.5C6.067 9.5 4.5 7.933 4.5 6Z",
            }
            path {
                d: "M8.99998 13.8546L6.85353 16.0011H6.14642L4.64642 14.5011L5.35353 13.7939L6.49998 14.9404L8.64642 12.7939H9.35353L11.5 14.9404L13.6464 12.7939H14.3535L16 14.4404V15.8546L14 13.8546L11.8535 16.0011H11.1464L8.99998 13.8546Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSearchStop;
impl IconShape for VsSearchStop {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.738 3.318a4.5 4.5 0 0 0-.877 5.123A4.48 4.48 0 0 0 6.1 10a4.62 4.62 0 0 0-.1 1v.17c-.16-.11-.32-.22-.47-.34L1.75 14.5 1 13.84l3.8-3.69a5.5 5.5 0 1 1 9.62-3.65c0 .268-.02.535-.06.8a5.232 5.232 0 0 0-.94-.68V6.5a4.5 4.5 0 0 0-7.682-3.182zm3.04 4.356a4 4 0 1 1 4.444 6.652 4 4 0 0 1-4.444-6.652zm.1 5.447A3 3 0 0 0 11 14a3 3 0 0 0 1.74-.55L8.55 9.26A3 3 0 0 0 8 11a3 3 0 0 0 .879 2.121zm.382-4.57l4.19 4.189A3 3 0 0 0 14 11a3 3 0 0 0-3-3 3 3 0 0 0-1.74.55z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSearch;
impl IconShape for VsSearch {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.25 0a8.25 8.25 0 0 0-6.18 13.72L1 22.88l1.12 1 8.05-9.12A8.251 8.251 0 1 0 15.25.01V0zm0 15a6.75 6.75 0 1 1 0-13.5 6.75 6.75 0 0 1 0 13.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSend;
impl IconShape for VsSend {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1 1.91L1.78 1.5L15 7.44899V8.3999L1.78 14.33L1 13.91L2.58311 8L1 1.91ZM3.6118 8.5L2.33037 13.1295L13.5 7.8999L2.33037 2.83859L3.6118 7.43874L9 7.5V8.5H3.6118Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsServerEnvironment;
impl IconShape for VsServerEnvironment {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 3h4v1H6V3zm0 6h4v1H6V9zm0 2h4v1H6v-1zm9.14 5H.86l1.25-5H4V2a.95.95 0 0 1 .078-.383c.052-.12.123-.226.211-.32a.922.922 0 0 1 .32-.219A1.01 1.01 0 0 1 5 1h6a.95.95 0 0 1 .383.078c.12.052.226.123.32.211a.922.922 0 0 1 .219.32c.052.125.078.256.078.391v9h1.89l1.25 5zM5 13h6V2H5v11zm8.86 2l-.75-3H12v2H4v-2H2.89l-.75 3h11.72z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsServerProcess;
impl IconShape for VsServerProcess {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 2h13l.5.5V9h-1V6H2v7h7v1H1.5l-.5-.5v-11l.5-.5zM2 5h12V3H2v2zm5 7v-1.094a1.633 1.633 0 0 1-.469-.265l-.945.539-.5-.86.937-.547a1.57 1.57 0 0 1 0-.547l-.937-.546.5-.86.945.54c.151-.12.308-.209.469-.266V7h1v1.094a1.48 1.48 0 0 1 .469.265l.945-.539.5.86-.937.547a1.57 1.57 0 0 1 0 .546l.937.547-.5.86-.945-.54a1.807 1.807 0 0 1-.469.266V12H7zm-.25-2.5c0 .208.073.385.219.531a.723.723 0 0 0 .531.219.723.723 0 0 0 .531-.219.723.723 0 0 0 .219-.531.723.723 0 0 0-.219-.531.723.723 0 0 0-.531-.219.723.723 0 0 0-.531.219.723.723 0 0 0-.219.531zm5.334 5.5v-1.094a1.634 1.634 0 0 1-.469-.265l-.945.539-.5-.86.938-.547a1.572 1.572 0 0 1 0-.547l-.938-.546.5-.86.945.54c.151-.12.308-.209.47-.266V10h1v1.094a1.486 1.486 0 0 1 .468.265l.945-.539.5.86-.937.547a1.562 1.562 0 0 1 0 .546l.937.547-.5.86-.945-.54a1.806 1.806 0 0 1-.469.266V15h-1zm-.25-2.5c0 .208.073.385.219.531a.723.723 0 0 0 .531.219.723.723 0 0 0 .531-.219.723.723 0 0 0 .22-.531.723.723 0 0 0-.22-.531.723.723 0 0 0-.53-.219.723.723 0 0 0-.532.219.723.723 0 0 0-.219.531z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsServer;
impl IconShape for VsServer {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.5 5L2 4.5v-3l.5-.5h11l.5.5v3l-.5.5h-11zM10 2H9v1H8V2H7v1H6V2H5v1H4V2H3v2h10V2h-2v1h-1V2zm-7.5 8L2 9.5v-3l.5-.5h11l.5.5v3l-.5.5h-11zM6 7H5v1H4V7H3v2h10V7h-2v1h-1V7H9v1H8V7H7v1H6V7zm7.5 8l.5-.5v-3l-.5-.5h-11l-.5.5v3l.5.5h11zM3 14v-2h1v1h1v-1h1v1h1v-1h1v1h1v-1h1v1h1v-1h2v2H3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSettingsGear;
impl IconShape for VsSettingsGear {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M19.85 8.75l4.15.83v4.84l-4.15.83 2.35 3.52-3.43 3.43-3.52-2.35-.83 4.15H9.58l-.83-4.15-3.52 2.35-3.43-3.43 2.35-3.52L0 14.42V9.58l4.15-.83L1.8 5.23 5.23 1.8l3.52 2.35L9.58 0h4.84l.83 4.15 3.52-2.35 3.43 3.43-2.35 3.52zm-1.57 5.07l4-.81v-2l-4-.81-.54-1.3 2.29-3.43-1.43-1.43-3.43 2.29-1.3-.54-.81-4h-2l-.81 4-1.3.54-3.43-2.29-1.43 1.43L6.38 8.9l-.54 1.3-4 .81v2l4 .81.54 1.3-2.29 3.43 1.43 1.43 3.43-2.29 1.3.54.81 4h2l.81-4 1.3-.54 3.43 2.29 1.43-1.43-2.29-3.43.54-1.3zm-8.186-4.672A3.43 3.43 0 0 1 12 8.57 3.44 3.44 0 0 1 15.43 12a3.43 3.43 0 1 1-5.336-2.852zm.956 4.274c.281.188.612.288.95.288A1.7 1.7 0 0 0 13.71 12a1.71 1.71 0 1 0-2.66 1.422z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSettings;
impl IconShape for VsSettings {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 9.5C6.93191 9.5 7.71496 10.1374 7.93699 11H13.5C13.7761 11 14 11.2239 14 11.5C14 11.7455 13.8231 11.9496 13.5899 11.9919L13.5 12L7.93673 12.001C7.71435 12.8631 6.93155 13.5 6 13.5C5.06845 13.5 4.28565 12.8631 4.06327 12.001L2.5 12C2.22386 12 2 11.7761 2 11.5C2 11.2545 2.17688 11.0504 2.41012 11.0081L2.5 11H4.06301C4.28504 10.1374 5.06809 9.5 6 9.5ZM6 10.5C5.44772 10.5 5 10.9477 5 11.5C5 12.0523 5.44772 12.5 6 12.5C6.55228 12.5 7 12.0523 7 11.5C7 10.9477 6.55228 10.5 6 10.5ZM10 2.5C10.9319 2.5 11.715 3.13738 11.937 3.99998L13.5 4C13.7761 4 14 4.22386 14 4.5C14 4.74546 13.8231 4.94961 13.5899 4.99194L13.5 5L11.9367 5.00102C11.7144 5.86312 10.9316 6.5 10 6.5C9.06845 6.5 8.28565 5.86312 8.06327 5.00102L2.5 5C2.22386 5 2 4.77614 2 4.5C2 4.25454 2.17688 4.05039 2.41012 4.00806L2.5 4L8.06301 3.99998C8.28504 3.13738 9.06809 2.5 10 2.5ZM10 3.5C9.44772 3.5 9 3.94772 9 4.5C9 5.05228 9.44772 5.5 10 5.5C10.5523 5.5 11 5.05228 11 4.5C11 3.94772 10.5523 3.5 10 3.5Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsShare;
impl IconShape for VsShare {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 4.00098H1.5L1 4.50098V14.501L1.5 15.001H12.5L13 14.501V11.5H12V14.001H2V5.00098H5V4.00098Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M6 11H5V8.5C5 6.08104 6.71776 4.06329 9 3.60002L9.00001 2.34885C9.00001 1.6039 9.60391 1 10.3489 1C10.7217 1 11.0779 1.15432 11.3329 1.42632L14.9795 5.31606V6.68394L11.3329 10.5737C11.0779 10.8457 10.7217 11 10.3489 11C9.60391 11 9 10.3961 9 9.65114L9 8.60178C8.06714 8.81236 7.61607 9.31244 7.36824 9.74614C7.18768 10.0621 7.09298 10.3875 7.04484 10.6402C7.02115 10.7646 7.00983 10.8656 7.0045 10.931C7.00184 10.9635 7.00072 10.9866 7.00025 10.9988L7.00019 11.0007C6.82645 11.0003 6.16755 11 6 11ZM10 4.5C7.89378 4.5 6.16778 6.12788 6.01152 8.1941C6.00389 8.29507 6 8.39708 6 8.5V10H6.1758C6.31381 9.55711 6.56019 9.03517 7 8.58307C7.44225 8.12846 8.08008 7.74446 9 7.58244C9.30193 7.52926 9.63424 7.5 10 7.5L10 9.65114C10 9.84381 10.1562 10 10.3489 10C10.4453 10 10.5374 9.96009 10.6034 9.88974L14.25 6L10.6034 2.11026C10.5374 2.03991 10.4453 2 10.3489 2C10.1562 2 10 2.15619 10 2.34886L10 4.5Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M7.00019 11.0007C7.05203 11.0008 7.06069 11.0006 7.00019 11.0007Z",
            }
            path {
                d: "M7.00019 11.0007L6.99996 11.0079L6.99997 11.0065L6.99999 11.0039L7 11.0021L7.00019 11.0007V11.0007Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsShield;
impl IconShape for VsShield {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.246 14.713a27.792 27.792 0 0 1-1.505-.953c-.501-.34-.983-.707-1.444-1.1-.458-.395-.888-.82-1.288-1.274-.4-.455-.753-.95-1.05-1.478a7.8 7.8 0 0 1-.7-1.69A7.041 7.041 0 0 1 2 6.3V3.1l.5-.5c.333 0 .656-.011.97-.036.296-.023.591-.066.882-.128.284-.062.562-.148.832-.256.284-.118.557-.261.816-.427a4.83 4.83 0 0 1 1.184-.565 4.8 4.8 0 0 1 2-.142 4.018 4.018 0 0 1 1.237.383c.199.097.392.204.58.322.26.167.535.31.821.428.27.109.547.194.831.256.291.062.587.106.884.129.311.024.634.035.967.035l.5.5v3.2a7.043 7.043 0 0 1-.256 1.919 7.804 7.804 0 0 1-.7 1.69 8.751 8.751 0 0 1-1.05 1.478c-.4.452-.829.877-1.286 1.27a15.94 15.94 0 0 1-1.448 1.1 28.71 28.71 0 0 1-1.51.956h-.508zM3 3.59V6.3c-.004.555.07 1.11.22 1.645a6.7 6.7 0 0 0 .61 1.473c.263.467.575.905.93 1.308.37.417.766.81 1.188 1.174.432.368.883.712 1.352 1.03.4.267.8.523 1.2.769.4-.242.8-.498 1.2-.768.47-.319.923-.663 1.355-1.031.421-.364.817-.756 1.186-1.172a7.8 7.8 0 0 0 .93-1.308c.261-.465.466-.96.61-1.473.15-.537.223-1.09.22-1.647V3.59c-.159 0-.313-.012-.465-.023l-.079-.006a7.95 7.95 0 0 1-1.018-.147 6.112 6.112 0 0 1-1.976-.814 5.166 5.166 0 0 0-.482-.27 3.123 3.123 0 0 0-.943-.29 3.686 3.686 0 0 0-1.558.106c-.332.108-.649.26-.94.452-.312.2-.64.372-.983.513a6.4 6.4 0 0 1-1 .307c-.335.07-.675.12-1.017.146-.174.01-.355.02-.54.026zm6.065 4.3a1.5 1.5 0 1 0-1.13 0L7.5 10.5h2l-.435-2.61z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSignIn;
impl IconShape for VsSignIn {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.02 3.77l.01-.01.99.99V2.5l-.5-.5h-9l-.51.5v.493L2 3v10.29l.36.46 5 1.72L8 15v-1h3.52l.5-.5v-2.25l-1 1V13H8V4.71l-.33-.46L4.036 3h6.984v.77zM7 14.28l-4-1.34V3.72l4 1.34v9.22zm3.09-6.75h4.97v1h-4.93l1.59 1.6-.71.7-2.47-2.46v-.71l2.49-2.48.7.7-1.64 1.65z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSignOut;
impl IconShape for VsSignOut {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.02 3.77v1.56l1-.99V2.5l-.5-.5h-9l-.5.5v.486L2 3v10.29l.36.46 5 1.72L8 15v-1h3.52l.5-.5v-1.81l-1-1V13H8V4.71l-.33-.46L4.036 3h6.984v.77zM7 14.28l-4-1.34V3.72l4 1.34v9.22zm6.52-5.8H8.55v-1h4.93l-1.6-1.6.71-.7 2.47 2.46v.71l-2.49 2.48-.7-.7 1.65-1.65z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSmiley;
impl IconShape for VsSmiley {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4.111 2.18a7 7 0 1 1 7.778 11.64A7 7 0 0 1 4.11 2.18zm.556 10.809a6 6 0 1 0 6.666-9.978 6 6 0 0 0-6.666 9.978zM6.5 7a1 1 0 1 1-2 0 1 1 0 0 1 2 0zm5 0a1 1 0 1 1-2 0 1 1 0 0 1 2 0zM8 11a3 3 0 0 1-2.65-1.58l-.87.48a4 4 0 0 0 7.12-.16l-.9-.43A3 3 0 0 1 8 11z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSnake;
impl IconShape for VsSnake {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 3.5C8 3.77614 7.77614 4 7.5 4C7.22386 4 7 3.77614 7 3.5C7 3.22386 7.22386 3 7.5 3C7.77614 3 8 3.22386 8 3.5Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M5.5 1C4.11929 1 3 2.11929 3 3.5C3 4.00954 3.15244 4.48348 3.4142 4.8787L2.29289 6H0.5V7H2V8.5H3V6.70711L4.1213 5.5858C4.51652 5.84756 4.99046 6 5.5 6H7V8H6C4.89543 8 4 8.89543 4 10C2.89543 10 2 10.8954 2 12C2 13.1046 2.89543 14 4 14H13C14.1046 14 15 13.1046 15 12C15 10.8954 14.1046 10 13 10C13 8.89543 12.1046 8 11 8V4.5C11 2.567 9.433 1 7.5 1H5.5ZM4 3.5C4 2.67157 4.67157 2 5.5 2H7.5C8.88071 2 10 3.11929 10 4.5V8.5L10.5 9H11C11.5523 9 12 9.44772 12 10V11H13C13.5523 11 14 11.4477 14 12C14 12.5523 13.5523 13 13 13H4C3.44772 13 3 12.5523 3 12C3 11.4477 3.44772 11 4 11H5V10C5 9.44772 5.44772 9 6 9H7.5L8 8.5V5.5L7.5 5H5.5C4.67157 5 4 4.32843 4 3.5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSortPrecedence;
impl IconShape for VsSortPrecedence {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7 2L6 3v3h1V3h7v2.453l.207-.16.793.793V3l-1-1H7zm1 2h2v2H8V4zM5 9H3v2h2V9zM2 7L1 8v5l1 1h7l1-1V8L9 7H2zm0 6V8h7v5H2zm6-3H6v2h2v-2zm5-6h-1v3.864l-1.182-1.182-.707.707 2.035 2.036h.708l2.035-2.036-.707-.707L13 7.864V4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSourceControl;
impl IconShape for VsSourceControl {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21.007 8.222A3.738 3.738 0 0 0 15.045 5.2a3.737 3.737 0 0 0 1.156 6.583 2.988 2.988 0 0 1-2.668 1.67h-2.99a4.456 4.456 0 0 0-2.989 1.165V7.4a3.737 3.737 0 1 0-1.494 0v9.117a3.776 3.776 0 1 0 1.816.099 2.99 2.99 0 0 1 2.668-1.667h2.99a4.484 4.484 0 0 0 4.223-3.039 3.736 3.736 0 0 0 3.25-3.687zM4.565 3.738a2.242 2.242 0 1 1 4.484 0 2.242 2.242 0 0 1-4.484 0zm4.484 16.441a2.242 2.242 0 1 1-4.484 0 2.242 2.242 0 0 1 4.484 0zm8.221-9.715a2.242 2.242 0 1 1 0-4.485 2.242 2.242 0 0 1 0 4.485z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSparkleFilled;
impl IconShape for VsSparkleFilled {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M 5.398 10.807 C 5.574 10.931 5.785 10.998 6 10.997 C 6.216 10.998 6.427 10.93 6.602 10.804 C 6.78 10.674 6.915 10.494 6.989 10.286 L 7.436 8.913 C 7.551 8.569 7.744 8.256 8 7.999 C 8.257 7.743 8.569 7.549 8.913 7.434 L 10.304 6.983 C 10.456 6.929 10.594 6.84 10.706 6.724 C 10.817 6.608 10.901 6.467 10.949 6.313 C 10.998 6.159 11.01 5.996 10.985 5.837 C 10.96 5.677 10.898 5.526 10.804 5.394 C 10.67 5.208 10.479 5.071 10.26 5.003 L 8.885 4.556 C 8.541 4.442 8.228 4.249 7.971 3.993 C 7.714 3.736 7.52 3.424 7.405 3.079 L 6.953 1.691 C 6.881 1.489 6.748 1.314 6.571 1.191 C 6.439 1.098 6.286 1.036 6.125 1.012 C 5.965 0.987 5.801 1.001 5.646 1.051 C 5.492 1.101 5.351 1.187 5.236 1.301 C 5.12 1.415 5.033 1.555 4.98 1.708 L 4.523 3.108 C 4.409 3.443 4.22 3.748 3.97 3.999 C 3.721 4.25 3.418 4.441 3.083 4.557 L 1.692 5.005 C 1.541 5.06 1.404 5.149 1.292 5.265 C 1.18 5.381 1.097 5.521 1.048 5.675 C 1 5.829 0.988 5.992 1.013 6.151 C 1.038 6.31 1.099 6.462 1.192 6.593 C 1.32 6.773 1.501 6.908 1.709 6.979 L 3.083 7.424 C 3.524 7.571 3.91 7.845 4.193 8.212 C 4.356 8.423 4.481 8.66 4.564 8.912 L 5.016 10.303 C 5.088 10.507 5.222 10.683 5.398 10.807 Z M 11.535 14.849 C 11.671 14.946 11.834 14.997 12 14.997 C 12.165 14.997 12.326 14.946 12.461 14.851 C 12.601 14.753 12.706 14.613 12.761 14.451 L 13.009 13.689 C 13.063 13.531 13.152 13.387 13.269 13.268 C 13.387 13.15 13.531 13.061 13.689 13.009 L 14.461 12.757 C 14.619 12.703 14.756 12.6 14.852 12.464 C 14.926 12.361 14.974 12.242 14.992 12.117 C 15.011 11.992 14.999 11.865 14.959 11.745 C 14.918 11.625 14.85 11.516 14.76 11.428 C 14.669 11.34 14.559 11.274 14.438 11.236 L 13.674 10.987 C 13.516 10.935 13.372 10.846 13.254 10.729 C 13.136 10.611 13.047 10.467 12.994 10.309 L 12.742 9.536 C 12.689 9.379 12.586 9.242 12.449 9.146 C 12.347 9.073 12.23 9.025 12.106 9.006 C 11.982 8.987 11.855 8.998 11.736 9.037 C 11.616 9.076 11.508 9.142 11.419 9.231 C 11.33 9.319 11.264 9.427 11.224 9.546 L 10.977 10.308 C 10.925 10.466 10.838 10.61 10.721 10.728 C 10.607 10.845 10.467 10.934 10.312 10.987 L 9.539 11.239 C 9.38 11.293 9.242 11.396 9.145 11.533 C 9.047 11.669 8.995 11.833 8.996 12.001 C 8.997 12.169 9.051 12.333 9.15 12.468 C 9.249 12.604 9.388 12.705 9.547 12.757 L 10.31 13.004 C 10.469 13.058 10.614 13.147 10.732 13.265 C 10.851 13.384 10.939 13.528 10.99 13.687 L 11.243 14.461 C 11.298 14.618 11.4 14.753 11.535 14.849 Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSparkle;
impl IconShape for VsSparkle {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.39804 10.8069C5.57428 10.9312 5.78476 10.9977 6.00043 10.9973C6.21633 10.9975 6.42686 10.93 6.60243 10.8043C6.77993 10.6739 6.91464 10.4936 6.98943 10.2863L7.43643 8.91335C7.55086 8.56906 7.74391 8.25615 8.00028 7.99943C8.25665 7.74272 8.56929 7.54924 8.91343 7.43435L10.3044 6.98335C10.4564 6.92899 10.5936 6.84019 10.7055 6.7239C10.8174 6.60762 10.9008 6.467 10.9492 6.31308C10.9977 6.15916 11.0098 5.99611 10.9847 5.83672C10.9596 5.67732 10.8979 5.52591 10.8044 5.39435C10.6703 5.20842 10.4794 5.07118 10.2604 5.00335L8.88543 4.55635C8.54091 4.44212 8.22777 4.24915 7.97087 3.99277C7.71396 3.73638 7.52035 3.42363 7.40543 3.07935L6.95343 1.69135C6.88113 1.48904 6.74761 1.31428 6.57143 1.19135C6.43877 1.09762 6.28607 1.03614 6.12548 1.01179C5.96489 0.987448 5.80083 1.00091 5.64636 1.05111C5.49188 1.1013 5.35125 1.18685 5.23564 1.30095C5.12004 1.41505 5.03265 1.55454 4.98043 1.70835L4.52343 3.10835C4.40884 3.44317 4.21967 3.74758 3.97022 3.9986C3.72076 4.24962 3.41753 4.44067 3.08343 4.55735L1.69243 5.00535C1.54065 5.05974 1.40352 5.14852 1.29177 5.26474C1.18001 5.38095 1.09666 5.52145 1.04824 5.67523C0.999819 5.82902 0.987639 5.99192 1.01265 6.1512C1.03767 6.31048 1.0992 6.46181 1.19243 6.59335C1.32027 6.7728 1.50105 6.90777 1.70943 6.97935L3.08343 7.42435C3.52354 7.57083 3.90999 7.84518 4.19343 8.21235C4.35585 8.42298 4.4813 8.65968 4.56443 8.91235L5.01643 10.3033C5.08846 10.5066 5.22179 10.6826 5.39804 10.8069ZM5.48343 3.39235L6.01043 2.01535L6.44943 3.39235C6.61312 3.8855 6.88991 4.33351 7.25767 4.70058C7.62544 5.06765 8.07397 5.34359 8.56743 5.50635L9.97343 6.03535L8.59143 6.48335C8.09866 6.64764 7.65095 6.92451 7.28382 7.29198C6.9167 7.65945 6.64026 8.10742 6.47643 8.60035L5.95343 9.97835L5.50443 8.59935C5.34335 8.10608 5.06943 7.65718 4.70443 7.28835C4.3356 6.92031 3.88653 6.64272 3.39243 6.47735L2.01443 5.95535L3.40043 5.50535C3.88672 5.33672 4.32775 5.05855 4.68943 4.69235C5.04901 4.32464 5.32049 3.88016 5.48343 3.39235ZM11.5353 14.8494C11.6713 14.9456 11.8337 14.9973 12.0003 14.9974C12.1654 14.9974 12.3264 14.9464 12.4613 14.8514C12.6008 14.7529 12.7058 14.6129 12.7613 14.4514L13.0093 13.6894C13.0625 13.5309 13.1515 13.3869 13.2693 13.2684C13.3867 13.1498 13.5307 13.0611 13.6893 13.0094L14.4613 12.7574C14.619 12.7029 14.7557 12.6004 14.8523 12.4644C14.9257 12.3614 14.9736 12.2424 14.9921 12.1173C15.0106 11.9922 14.9992 11.8645 14.9588 11.7447C14.9184 11.6249 14.8501 11.5163 14.7597 11.428C14.6692 11.3396 14.5591 11.2739 14.4383 11.2364L13.6743 10.9874C13.5162 10.9348 13.3724 10.8462 13.2544 10.7285C13.1364 10.6109 13.0473 10.4674 12.9943 10.3094L12.7423 9.53638C12.6886 9.37853 12.586 9.24191 12.4493 9.14638C12.3473 9.07343 12.2295 9.02549 12.1056 9.00642C11.9816 8.98736 11.8549 8.99772 11.7357 9.03665C11.6164 9.07558 11.508 9.142 11.4192 9.23054C11.3304 9.31909 11.2636 9.42727 11.2243 9.54638L10.9773 10.3084C10.925 10.466 10.8375 10.6097 10.7213 10.7284C10.6066 10.8449 10.4667 10.9335 10.3123 10.9874L9.53931 11.2394C9.38025 11.2933 9.2422 11.3959 9.1447 11.5326C9.04721 11.6694 8.99522 11.8333 8.99611 12.0013C8.99699 12.1692 9.0507 12.3326 9.14963 12.4683C9.24856 12.604 9.38769 12.7051 9.54731 12.7574L10.3103 13.0044C10.4692 13.0578 10.6136 13.1471 10.7323 13.2654C10.8505 13.3836 10.939 13.5283 10.9903 13.6874L11.2433 14.4614C11.2981 14.6178 11.4001 14.7534 11.5353 14.8494ZM10.6223 12.0564L10.4433 11.9974L10.6273 11.9334C10.9291 11.8284 11.2027 11.6556 11.4273 11.4284C11.6537 11.1994 11.8248 10.9216 11.9273 10.6164L11.9853 10.4384L12.0443 10.6194C12.1463 10.9261 12.3185 11.2047 12.5471 11.4332C12.7757 11.6617 13.0545 11.8336 13.3613 11.9354L13.5563 11.9984L13.3763 12.0574C13.0689 12.1596 12.7898 12.3322 12.5611 12.5616C12.3324 12.791 12.1606 13.0707 12.0593 13.3784L12.0003 13.5594L11.9423 13.3784C11.8409 13.0702 11.6687 12.7901 11.4394 12.5605C11.2102 12.3309 10.9303 12.1583 10.6223 12.0564Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSplitHorizontal;
impl IconShape for VsSplitHorizontal {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 1H3L2 2v11l1 1h11l1-1V2l-1-1zM8 13H3V2h5v11zm6 0H9V2h5v11z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSplitVertical;
impl IconShape for VsSplitVertical {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 1H3L2 2v11l1 1h11l1-1V2l-1-1zm0 12H3V8h11v5zm0-6H3V2h11v5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSquirrel;
impl IconShape for VsSquirrel {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.558 2.642a3.698 3.698 0 0 0-.123-.01A1.47 1.47 0 0 0 3.999 1.52v1.307a4.898 4.898 0 0 0-2.993 3.587v.39c.459.836 1.906 1.13 2.154 1.18.027.006.04.009.035.009-2.419.32-2.19 2.249-2.19 2.249a1 1 0 0 0 1 .93c.272-.019.538-.08.79-.18h2.06a3 3 0 0 0-.36 1h-.32a2.55 2.55 0 0 0-2.17 2.528.42.42 0 0 0 .39.48h6.677a3.76 3.76 0 0 0 3.929-4.158 3.649 3.649 0 0 0-.75-2.09l-.11-.14c-.43-.55-.68-.909-.55-1.289.13-.38.365-.4.365-.4s.185-.03.455.09c.22.128.46.22.71.27a1.58 1.58 0 0 0 1.736-.905c.095-.208.143-.435.143-.664.006-.718-.33-1.455-.725-2.088a4.998 4.998 0 0 0-1.554-1.57 3.998 3.998 0 0 0-2.639-.4 3.049 3.049 0 0 0-1.67.89 3.56 3.56 0 0 0-.779 1.359 4.358 4.358 0 0 0-.636-.747v-.159A1.47 1.47 0 0 0 5.558 1.52v1.122zm5.304 8.739c.111.741.22 1.821-.867 2.442-.296.103-.608.16-.923.167H3.215a1 1 0 0 1 .92-1h1.279v-.499a1.79 1.79 0 0 1 1.653-1.825l-.626-.887c-.236.067-.463.153-.577.233H2.655a.754.754 0 0 0-.264.07c-.138.055-.274.109-.396.03-.2-.13.11-1.12 1.01-1.12h1c.49 0 .57-.54.57-.54l.28-1.129a3.389 3.389 0 0 1-2.85-.93 3.389 3.389 0 0 1 3.14-2.658l.083.002c.26.008.435.014.776.168.93.42 2.149 2.469 2.149 2.469l.06.09h.17v-.07c-.06-.443-.02-1.464.116-1.89.137-.424.367-.814.673-1.14a2.349 2.349 0 0 1 1.3-.659 2.639 2.639 0 0 1 1.86.29c.46.284.85.67 1.139 1.127.289.457.476.836.535 1.374-.001.02 0 .047.002.081.007.143.02.39-.128.547-.127.135-.448.23-.67.18a1.57 1.57 0 0 1-.45-.18 1.33 1.33 0 0 0-1.139-.13 1.42 1.42 0 0 0-.94 1 2.318 2.318 0 0 0 .64 2.238l.11.14c.347.434.546.966.57 1.52a2.999 2.999 0 0 1-.306 1.425 2.708 2.708 0 0 0-.464-1.304l-.37.368zM4.24 5a.5.5 0 1 0 0 1 .5.5 0 0 0 0-1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsStarEmpty;
impl IconShape for VsStarEmpty {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.1939 2.1017C7.52403 1.43278 8.47789 1.43277 8.80802 2.1017L10.3291 5.18375L13.7304 5.67798C14.4685 5.78525 14.7633 6.69242 14.2291 7.2131L11.768 9.61215L12.349 12.9997C12.4751 13.7349 11.7034 14.2955 11.0431 13.9484L8.00096 12.349L4.95879 13.9484C4.29853 14.2955 3.52684 13.7349 3.65294 12.9997L4.23394 9.61215L1.77277 7.2131C1.23861 6.69242 1.53336 5.78525 2.27156 5.67798L5.67281 5.18375L7.1939 2.1017ZM8.00096 2.72596L6.54628 5.67346C6.41519 5.93909 6.16178 6.1232 5.86864 6.1658L2.61588 6.63845L4.9696 8.93276C5.18171 9.13952 5.27851 9.43742 5.22843 9.72938L4.6728 12.969L7.58215 11.4395C7.84434 11.3016 8.15758 11.3016 8.41977 11.4395L11.3291 12.969L10.7735 9.72938C10.7234 9.43742 10.8202 9.13952 11.0323 8.93276L13.386 6.63845L10.1333 6.1658C9.84014 6.1232 9.58673 5.93909 9.45564 5.67346L8.00096 2.72596Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsStarFull;
impl IconShape for VsStarFull {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.1939 2.1017C7.52403 1.43278 8.47789 1.43277 8.80802 2.1017L10.3291 5.18375L13.7304 5.67798C14.4685 5.78525 14.7633 6.69242 14.2291 7.2131L11.768 9.61215L12.349 12.9997C12.4751 13.7349 11.7034 14.2955 11.0431 13.9484L8.00096 12.349L4.95879 13.9484C4.29853 14.2955 3.52684 13.7349 3.65294 12.9997L4.23394 9.61215L1.77277 7.2131C1.23861 6.69242 1.53336 5.78525 2.27156 5.67798L5.67281 5.18375L7.1939 2.1017Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsStarHalf;
impl IconShape for VsStarHalf {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.67281 5.18372L7.1939 2.10167C7.3588 1.76754 7.67932 1.60031 8 1.59998C8.32132 1.59964 8.64279 1.76687 8.80802 2.10167L10.3291 5.18372L13.7304 5.67795C14.4685 5.78522 14.7633 6.69239 14.2291 7.21307L11.768 9.61212L12.349 12.9996C12.4751 13.7348 11.7034 14.2955 11.0431 13.9484L8.00096 12.349L4.95879 13.9484C4.29853 14.2955 3.52684 13.7348 3.65294 12.9996L4.23394 9.61212L1.77277 7.21307C1.23861 6.69239 1.53336 5.78522 2.27156 5.67795L5.67281 5.18372ZM8 11.336C8.14418 11.3359 8.28838 11.3703 8.41977 11.4394L11.3291 12.969L10.7735 9.72935C10.7234 9.43739 10.8202 9.13949 11.0323 8.93273L13.386 6.63842L10.1333 6.16577C9.84014 6.12317 9.58673 5.93906 9.45564 5.67343L8.00096 2.72593L8 2.72787V11.336Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsStopCircle;
impl IconShape for VsStopCircle {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 6h4v4H6z",
            }
            path {
                clip_rule: "evenodd",
                d: "M8.6 1c1.6.1 3.1.9 4.2 2 1.3 1.4 2 3.1 2 5.1 0 1.6-.6 3.1-1.6 4.4-1 1.2-2.4 2.1-4 2.4-1.6.3-3.2.1-4.6-.7-1.4-.8-2.5-2-3.1-3.5C.9 9.2.8 7.5 1.3 6c.5-1.6 1.4-2.9 2.8-3.8C5.4 1.3 7 .9 8.6 1zm.5 12.9c1.3-.3 2.5-1 3.4-2.1.8-1.1 1.3-2.4 1.2-3.8 0-1.6-.6-3.2-1.7-4.3-1-1-2.2-1.6-3.6-1.7-1.3-.1-2.7.2-3.8 1-1.1.8-1.9 1.9-2.3 3.3-.4 1.3-.4 2.7.2 4 .6 1.3 1.5 2.3 2.7 3 1.2.7 2.6.9 3.9.6z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSurroundWith;
impl IconShape for VsSurroundWith {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 3.99988H3V2.99988H1.5L1 3.49988V12.4999L1.5 12.9999H3V11.9999H2V3.99988ZM14.5 2.99988H13V3.99988H14V11.9999H13V12.9999H14.5L15 12.4999V3.49988L14.5 2.99988ZM5 8.99988C5.55228 8.99988 6 8.55216 6 7.99988C6 7.44759 5.55228 6.99988 5 6.99988C4.44772 6.99988 4 7.44759 4 7.99988C4 8.55216 4.44772 8.99988 5 8.99988ZM9 7.99988C9 8.55216 8.55228 8.99988 8 8.99988C7.44772 8.99988 7 8.55216 7 7.99988C7 7.44759 7.44772 6.99988 8 6.99988C8.55228 6.99988 9 7.44759 9 7.99988ZM11 8.99988C11.5523 8.99988 12 8.55216 12 7.99988C12 7.44759 11.5523 6.99988 11 6.99988C10.4477 6.99988 10 7.44759 10 7.99988C10 8.55216 10.4477 8.99988 11 8.99988Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolArray;
impl IconShape for VsSymbolArray {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 2l-.5.5v11l.5.5H4v-1H2V3h2V2H1.5zm13 12l.5-.5v-11l-.5-.5H12v1h2v10h-2v1h2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolBoolean;
impl IconShape for VsSymbolBoolean {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1 3.5l.5-.5h13l.5.5v9l-.5.5h-13l-.5-.5v-9zM14 4H8v3.493h-.5l-3.574-.005 2.09-2.09-.707-.707-2.955 2.955v.708l2.955 2.955.707-.707-2.114-2.114 3.996.005H8v-.986l3.907.005-2.114-2.114.707-.707 2.956 2.955v.708L10.5 11.309l-.707-.707 2.09-2.09L8 8.507V12h6V4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolClass;
impl IconShape for VsSymbolClass {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.34 9.71h.71l2.67-2.67v-.71L13.38 5h-.7l-1.82 1.81h-5V5.56l1.86-1.85V3l-2-2H5L1 5v.71l2 2h.71l1.14-1.15v5.79l.5.5H10v.52l1.33 1.34h.71l2.67-2.67v-.71L13.37 10h-.7l-1.86 1.85h-5v-4H10v.48l1.34 1.38zm1.69-3.65l.63.63-2 2-.63-.63 2-2zm0 5l.63.63-2 2-.63-.63 2-2zM3.35 6.65l-1.29-1.3 3.29-3.29 1.3 1.29-3.3 3.3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolColor;
impl IconShape for VsSymbolColor {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8 1.003a7 7 0 0 0-7 7v.43c.09 1.51 1.91 1.79 3 .7a1.87 1.87 0 0 1 2.64 2.64c-1.1 1.16-.79 3.07.8 3.2h.6a7 7 0 1 0 0-14l-.04.03zm0 13h-.52a.58.58 0 0 1-.36-.14.56.56 0 0 1-.15-.3 1.24 1.24 0 0 1 .35-1.08 2.87 2.87 0 0 0 0-4 2.87 2.87 0 0 0-4.06 0 1 1 0 0 1-.9.34.41.41 0 0 1-.22-.12.42.42 0 0 1-.1-.29v-.37a6 6 0 1 1 6 6l-.04-.04zM9 3.997a1 1 0 1 1-2 0 1 1 0 0 1 2 0zm3 7.007a1 1 0 1 1-2 0 1 1 0 0 1 2 0zm-7-5a1 1 0 1 0 0-2 1 1 0 0 0 0 2zm7-1a1 1 0 1 1-2 0 1 1 0 0 1 2 0zM13 8a1 1 0 1 1-2 0 1 1 0 0 1 2 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolConstant;
impl IconShape for VsSymbolConstant {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 6h8v1H4V6zm8 3H4v1h8V9z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M1 4l1-1h12l1 1v8l-1 1H2l-1-1V4zm1 0v8h12V4H2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolEnumMember;
impl IconShape for VsSymbolEnumMember {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7 3l1-1h6l1 1v5l-1 1h-4V8h4V3H8v3H7V3zm2 6V8L8 7H2L1 8v5l1 1h6l1-1V9zM8 8v5H2V8h6zm1.414-1L9 6.586V6h4v1H9.414zM9 4h4v1H9V4zm-2 6H3v1h4v-1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolEnum;
impl IconShape for VsSymbolEnum {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14 2H8L7 3v3h1V3h6v5h-4v1h4l1-1V3l-1-1zM9 6h4v1H9.41L9 6.59V6zM7 7H2L1 8v5l1 1h6l1-1V8L8 7H7zm1 6H2V8h6v5zM3 9h4v1H3V9zm0 2h4v1H3v-1zm6-7h4v1H9V4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolEvent;
impl IconShape for VsSymbolEvent {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.414 1.56L8.312 1h3.294l.818 1.575L10.236 6h1.781l.72 1.695L5.618 15l-1.602-1.163L6.119 10H4.898L4 8.56l3.414-7zM7.78 9L4.9 14.305 12.018 7H8.312l3.294-5H8.312L4.898 9H7.78z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolField;
impl IconShape for VsSymbolField {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.45 4.5l-5-2.5h-.9l-7 3.5-.55.89v4.5l.55.9 5 2.5h.9l7-3.5.55-.9v-4.5l-.55-.89zm-8 8.64l-4.5-2.25V7.17l4.5 2v3.97zm.5-4.8L2.29 6.23l6.66-3.34 4.67 2.34-6.67 3.11zm7 1.55l-6.5 3.25V9.21l6.5-3v3.68z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolFile;
impl IconShape for VsSymbolFile {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.85 4.44l-3.28-3.3-.35-.14H2.5l-.5.5v13l.5.5h11l.5-.5V4.8l-.15-.36zM13 5h-3V2l3 3zM3 14V2h6v3.5l.5.5H13v8H3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolInterface;
impl IconShape for VsSymbolInterface {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.496 4a3.49 3.49 0 0 0-3.46 3h-3.1a2 2 0 1 0 0 1h3.1a3.5 3.5 0 1 0 3.46-4zm0 6a2.5 2.5 0 1 1 0-5 2.5 2.5 0 0 1 0 5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolKey;
impl IconShape for VsSymbolKey {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.223 10.933c.326.192.699.29 1.077.282a2.159 2.159 0 0 0 1.754-.842 3.291 3.291 0 0 0 .654-2.113 2.886 2.886 0 0 0-.576-1.877 1.99 1.99 0 0 0-1.634-.733 2.294 2.294 0 0 0-1.523.567V3.475h-.991V11.1h.995v-.344c.076.066.158.125.244.177zM7.85 6.7c.186-.079.388-.113.59-.1a1.08 1.08 0 0 1 .896.428c.257.363.382.802.357 1.245a2.485 2.485 0 0 1-.4 1.484 1.133 1.133 0 0 1-.96.508 1.224 1.224 0 0 1-.976-.417A1.522 1.522 0 0 1 6.975 8.8v-.6a1.722 1.722 0 0 1 .393-1.145c.13-.154.296-.276.482-.355zM3.289 5.675a3.03 3.03 0 0 0-.937.162 2.59 2.59 0 0 0-.8.4l-.1.077v1.2l.423-.359a2.1 2.1 0 0 1 1.366-.572.758.758 0 0 1 .661.282c.15.232.23.503.231.779L2.9 7.825a2.6 2.6 0 0 0-1.378.575 1.65 1.65 0 0 0-.022 2.336 1.737 1.737 0 0 0 1.253.454 1.96 1.96 0 0 0 1.107-.332c.102-.068.197-.145.286-.229v.444h.941V7.715a2.193 2.193 0 0 0-.469-1.5 1.687 1.687 0 0 0-1.329-.54zm.857 3.041c.02.418-.12.829-.391 1.148a1.221 1.221 0 0 1-.955.422.832.832 0 0 1-.608-.2.833.833 0 0 1 0-1.091c.281-.174.6-.277.93-.3l1.02-.148.004.169zm8.313 2.317c.307.13.64.193.973.182.495.012.983-.114 1.41-.365l.123-.075.013-.007V9.615l-.446.32c-.316.224-.696.34-1.084.329A1.3 1.3 0 0 1 12.4 9.8a1.975 1.975 0 0 1-.4-1.312 2.01 2.01 0 0 1 .453-1.381A1.432 1.432 0 0 1 13.6 6.6a1.8 1.8 0 0 1 .971.279l.43.265V5.97l-.17-.073a2.9 2.9 0 0 0-1.17-.247 2.52 2.52 0 0 0-1.929.817 2.9 2.9 0 0 0-.747 2.049c-.028.707.21 1.4.67 1.939.222.249.497.446.804.578z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolKeyword;
impl IconShape for VsSymbolKeyword {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 4h-5V3h5v1zm-1 3h-2v1h2V7zm-4 0H1v1h9V7zm2 6H1v1h11v-1zm-5-3H1v1h6v-1zm8 0h-5v1h5v-1zM8 2v3H1V2h7zM7 3H2v1h5V3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolMethodArrow;
impl IconShape for VsSymbolMethodArrow {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.51001 1L13.51 4L14 4.85999V9.08768C13.8397 9.03026 13.6706 9.00002 13.5 9.00002C13.3327 8.99915 13.1676 9.02685 13.01 9.0818V5.69995L8.51001 8.15002V13.5601L9.15838 13.171C9.22976 13.3138 9.3241 13.4457 9.439 13.561C9.57348 13.695 9.73047 13.8011 9.90109 13.8753L8.51001 14.71H7.51001L2.51001 11.71L2.02002 10.86V4.85999L2.51001 4L7.51001 1H8.51001ZM3.01001 10.86L7.51001 13.5601V8.15002L3.01001 5.69995V10.86ZM8.01001 1.85999L3.27002 4.69995L8.01001 7.29004L12.75 4.69995L8.01001 1.85999Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M13.85 14.85C13.805 14.897 13.751 14.935 13.69 14.961C13.63 14.987 13.565 15 13.5 15C13.435 15 13.37 14.987 13.31 14.961C13.25 14.935 13.196 14.897 13.15 14.85C13.103 14.805 13.065 14.751 13.039 14.69C13.013 14.63 13 14.565 13 14.5C13 14.435 13.013 14.37 13.039 14.31C13.065 14.25 13.103 14.196 13.15 14.15L14.29 13H10.5C10.367 13 10.24 12.947 10.146 12.854C10.052 12.761 10 12.633 10 12.5C10 12.367 10.053 12.24 10.146 12.146C10.24 12.052 10.367 12 10.5 12H14.29L13.15 10.85C13.059 10.757 13.008 10.631 13.009 10.501C13.009 10.371 13.062 10.246 13.154 10.153C13.246 10.061 13.371 10.009 13.502 10.008C13.632 10.008 13.758 10.058 13.851 10.149L15.851 12.149C15.898 12.194 15.936 12.248 15.962 12.309C15.988 12.369 16.001 12.434 16.001 12.499C16.001 12.564 15.988 12.629 15.962 12.689C15.936 12.749 15.898 12.803 15.851 12.849L13.85 14.85Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolMethod;
impl IconShape for VsSymbolMethod {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.51 4l-5-3h-1l-5 3-.49.86v6l.49.85 5 3h1l5-3 .49-.85v-6L13.51 4zm-6 9.56l-4.5-2.7V5.7l4.5 2.45v5.41zM3.27 4.7l4.74-2.84 4.74 2.84-4.74 2.59L3.27 4.7zm9.74 6.16l-4.5 2.7V8.15l4.5-2.45v5.16z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolMisc;
impl IconShape for VsSymbolMisc {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 2h8v4c.341.035.677.112 1 .23V1H3v8.48l1-1.75V2zm2.14 8L5 8 4 9.75 3.29 11 1 15h8l-2.29-4-.57-1zm-3.42 4l1.72-3L5 10l.56 1 1.72 3H2.72zm6.836-6.41a3.5 3.5 0 1 1 3.888 5.82 3.5 3.5 0 0 1-3.888-5.82zm.555 4.989a2.5 2.5 0 1 0 2.778-4.157 2.5 2.5 0 0 0-2.778 4.157z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolNamespace;
impl IconShape for VsSymbolNamespace {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2.984V2h-.09c-.313 0-.616.062-.909.185a2.33 2.33 0 0 0-.775.53 2.23 2.23 0 0 0-.493.753v.001a3.542 3.542 0 0 0-.198.83v.002a6.08 6.08 0 0 0-.024.863c.012.29.018.58.018.869 0 .203-.04.393-.117.572v.001a1.504 1.504 0 0 1-.765.787 1.376 1.376 0 0 1-.558.115H2v.984h.09c.195 0 .38.04.556.121l.001.001c.178.078.329.184.455.318l.002.002c.13.13.233.285.307.465l.001.002c.078.18.117.368.117.566 0 .29-.006.58-.018.869-.012.296-.004.585.024.87v.001c.033.283.099.558.197.824v.001c.106.273.271.524.494.753.223.23.482.407.775.53.293.123.596.185.91.185H6v-.984h-.09c-.199 0-.387-.038-.562-.115a1.613 1.613 0 0 1-.457-.32 1.659 1.659 0 0 1-.309-.467c-.074-.18-.11-.37-.11-.573 0-.228.003-.453.011-.672.008-.228.008-.45 0-.665a4.639 4.639 0 0 0-.055-.64 2.682 2.682 0 0 0-.168-.609A2.284 2.284 0 0 0 3.522 8a2.284 2.284 0 0 0 .738-.955c.08-.192.135-.393.168-.602.033-.21.051-.423.055-.64.008-.22.008-.442 0-.666-.008-.224-.012-.45-.012-.678a1.47 1.47 0 0 1 .877-1.354 1.33 1.33 0 0 1 .563-.121H6zm4 10.032V14h.09c.313 0 .616-.062.909-.185.293-.123.552-.3.775-.53.223-.23.388-.48.493-.753v-.001c.1-.266.165-.543.198-.83v-.002c.028-.28.036-.567.024-.863-.012-.29-.018-.58-.018-.869 0-.203.04-.393.117-.572v-.001a1.504 1.504 0 0 1 .765-.787c.176-.077.362-.115.558-.115H14v-.984h-.09c-.195 0-.38-.04-.556-.121l-.001-.001a1.376 1.376 0 0 1-.455-.318l-.002-.002a1.414 1.414 0 0 1-.307-.465l-.001-.002a1.405 1.405 0 0 1-.117-.566c0-.29.006-.58.018-.869a6.19 6.19 0 0 0-.024-.87v-.001a3.542 3.542 0 0 0-.197-.824v-.001a2.23 2.23 0 0 0-.494-.753 2.33 2.33 0 0 0-.775-.53 2.325 2.325 0 0 0-.91-.185H10v.984h.09c.2 0 .386.038.562.115.174.082.326.188.457.32.127.134.23.29.309.467.074.18.11.37.11.573 0 .228-.003.452-.011.672-.008.228-.008.45 0 .665.004.222.022.435.055.64.033.214.089.416.168.609a2.282 2.282 0 0 0 .738.955 2.282 2.282 0 0 0-.738.955 2.7 2.7 0 0 0-.168.602c-.033.21-.051.423-.055.64-.008.22-.008.442 0 .666.008.224.012.45.012.678a1.47 1.47 0 0 1-.42 1.035 1.466 1.466 0 0 1-.457.319 1.33 1.33 0 0 1-.563.121H10z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolNumeric;
impl IconShape for VsSymbolNumeric {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11 1v4h4v1h-4v4h4v1h-4v4h-1v-4H6v4H5v-4H1v-1h4V6H1V5h4V1h1v4h4V1h1zM6 6v4h4V6H6z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolOperator;
impl IconShape for VsSymbolOperator {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.873 1.1c.335.136.602.398.745.73.072.17.109.352.107.537a1.34 1.34 0 0 1-.61 1.135 1.359 1.359 0 0 1-.753.223A1.355 1.355 0 0 1 1 2.362a1.355 1.355 0 0 1 .83-1.256A1.37 1.37 0 0 1 2.873 1.1zm-.298 1.765a.551.551 0 0 0 .332-.5.548.548 0 1 0-.332.5zM6.43 1.109L1.11 6.43l.686.687 5.32-5.32-.686-.687zM11.5 9h1v2.5H15v1h-2.5V15h-1v-2.5H9v-1h2.5V9zm-5.732.525l.707.707L4.707 12l1.768 1.768-.707.707L4 12.707l-1.768 1.768-.707-.707L3.293 12l-1.768-1.768.707-.707L4 11.293l1.768-1.768zm1.35-4.195a1.353 1.353 0 0 0-1.256-.83 1.355 1.355 0 0 0-1.256.83 1.362 1.362 0 0 0 1.257 1.895A1.358 1.358 0 0 0 7.118 5.33zm-.753.745a.553.553 0 0 1-.289.29.547.547 0 0 1-.599-.117.529.529 0 0 1-.117-.173.544.544 0 0 1 .716-.715.565.565 0 0 1 .173.116.549.549 0 0 1 .116.599zM14 3h-4v1h4V3z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolParameter;
impl IconShape for VsSymbolParameter {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11 6h-1v-.5a.5.5 0 0 0-.5-.5H8.479v5.5a.5.5 0 0 0 .5.5h.5v1h-3v-1h.5a.5.5 0 0 0 .5-.5V5H6.5a.5.5 0 0 0-.5.5V6H5V4h6v2zm2.914 2.048l-1.462-1.462.707-.707 1.816 1.816v.707l-1.768 1.767-.707-.707 1.414-1.414zM3.548 9.462L2.086 8 3.5 6.586l-.707-.707-1.768 1.767v.708l1.816 1.815.707-.707z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolProperty;
impl IconShape for VsSymbolProperty {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.807 14.975a1.75 1.75 0 0 1-1.255-.556 1.684 1.684 0 0 1-.544-1.1A1.72 1.72 0 0 1 1.36 12.1c1.208-1.27 3.587-3.65 5.318-5.345a4.257 4.257 0 0 1 .048-3.078 4.095 4.095 0 0 1 1.665-1.969 4.259 4.259 0 0 1 4.04-.36l.617.268-2.866 2.951 1.255 1.259 2.944-2.877.267.619a4.295 4.295 0 0 1 .04 3.311 4.198 4.198 0 0 1-.923 1.392 4.27 4.27 0 0 1-.743.581 4.217 4.217 0 0 1-3.812.446c-1.098 1.112-3.84 3.872-5.32 5.254a1.63 1.63 0 0 1-1.084.423zm7.938-13.047a3.32 3.32 0 0 0-1.849.557c-.213.13-.412.284-.591.458a3.321 3.321 0 0 0-.657 3.733l.135.297-.233.227c-1.738 1.697-4.269 4.22-5.485 5.504a.805.805 0 0 0 .132 1.05.911.911 0 0 0 .298.22c.1.044.209.069.319.072a.694.694 0 0 0 .45-.181c1.573-1.469 4.612-4.539 5.504-5.44l.23-.232.294.135a3.286 3.286 0 0 0 3.225-.254 3.33 3.33 0 0 0 .591-.464 3.28 3.28 0 0 0 .964-2.358c0-.215-.021-.43-.064-.642L11.43 7.125 8.879 4.578l2.515-2.59a3.286 3.286 0 0 0-.65-.06z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolRuler;
impl IconShape for VsSymbolRuler {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 1L3 2v12l1 1h8l1-1V2l-1-1H4zm0 2V2h8v12H4v-1h2v-1H4v-2h4V9H4V7h2V6H4V4h4V3H4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolSnippet;
impl IconShape for VsSymbolSnippet {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.5 1l-.5.5V13h1V2h11v11h1V1.5l-.5-.5h-12zM2 15v-1h1v1H2zm3-1H4v1h1v-1zm1 0h1v1H6v-1zm3 0H8v1h1v-1zm1 0h1v1h-1v-1zm5 1v-1h-1v1h1zm-3-1h1v1h-1v-1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolString;
impl IconShape for VsSymbolString {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 2L1 3v9l1 1h12l1-1V3l-1-1H2zm0 10V3h12v9H2zm3.356-3.07H6V7.22C6 6.408 5.685 6 5.056 6c-.135 0-.285.024-.45.073a1.444 1.444 0 0 0-.388.167v.665c.237-.203.487-.304.75-.304.261 0 .392.156.392.469l-.6.103c-.506.086-.76.406-.76.961 0 .263.061.473.183.631A.61.61 0 0 0 4.69 9c.29 0 .509-.16.657-.48h.009v.41zm.004-1.355v.193a.75.75 0 0 1-.12.436.368.368 0 0 1-.313.17.276.276 0 0 1-.22-.095.38.38 0 0 1-.08-.248c0-.222.11-.351.332-.389l.4-.067zM7.6 8.626h-.007v.31H7V5h.593v1.677h.008c.146-.31.355-.465.625-.465.248 0 .44.118.573.353.134.236.201.557.201.966 0 .443-.078.798-.235 1.067C8.61 8.866 8.4 9 8.138 9c-.237 0-.416-.125-.537-.374zm-.016-1.121v.272a.78.78 0 0 0 .107.426c.071.113.163.169.274.169.135 0 .24-.072.314-.216.075-.145.113-.35.113-.615 0-.22-.035-.39-.104-.514-.067-.124-.164-.187-.29-.187-.12 0-.219.062-.298.185a.887.887 0 0 0-.116.48zM11.262 9c.321 0 .567-.058.738-.173v-.71a.9.9 0 0 1-.552.207.619.619 0 0 1-.5-.215c-.12-.145-.181-.345-.181-.598 0-.26.063-.464.189-.612a.644.644 0 0 1 .516-.223c.194 0 .37.069.528.207v-.749c-.129-.09-.338-.134-.626-.134-.417 0-.751.14-1.001.422-.249.28-.373.662-.373 1.148 0 .42.116.764.349 1.03.232.267.537.4.913.4z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolStructure;
impl IconShape for VsSymbolStructure {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 2L1 3v3l1 1h12l1-1V3l-1-1H2zm0 1h12v3H2V3zm-1 7l1-1h3l1 1v3l-1 1H2l-1-1v-3zm2 0H2v3h3v-3H3zm7 0l1-1h3l1 1v3l-1 1h-3l-1-1v-3zm2 0h-1v3h3v-3h-2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSymbolVariable;
impl IconShape for VsSymbolVariable {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 5h2V4H1.5l-.5.5v8l.5.5H4v-1H2V5zm12.5-1H12v1h2v7h-2v1h2.5l.5-.5v-8l-.5-.5zm-2.74 2.57L12 7v2.51l-.3.45-4.5 2h-.46l-2.5-1.5-.24-.43v-2.5l.3-.46 4.5-2h.46l2.5 1.5zM5 9.71l1.5.9V9.28L5 8.38v1.33zm.58-2.15l1.45.87 3.39-1.5-1.45-.87-3.39 1.5zm1.95 3.17l3.5-1.56v-1.4l-3.5 1.55v1.41z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSyncIgnored;
impl IconShape for VsSyncIgnored {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.468 3.687l-.757-.706a6 6 0 0 1 9.285 4.799L15.19 6.6l.75.76-2.09 2.07-.76-.01L11 7.31l.76-.76 1.236 1.25a5 5 0 0 0-7.528-4.113zm4.55 8.889l.784.73a6 6 0 0 1-8.796-5.04L.78 9.5 0 8.73l2.09-2.07.76.01 2.09 2.12-.76.76-1.167-1.18a5 5 0 0 0 7.005 4.206z",
                fill_rule: "evenodd",
            }
            path {
                d: "M1.123 2.949l.682-.732L13.72 13.328l-.682.732z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsSync;
impl IconShape for VsSync {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.006 8.267L.78 9.5 0 8.73l2.09-2.07.76.01 2.09 2.12-.76.76-1.167-1.18a5 5 0 0 0 9.4 1.983l.813.597a6 6 0 0 1-11.22-2.683zm10.99-.466L11.76 6.55l-.76.76 2.09 2.11.76.01 2.09-2.07-.75-.76-1.194 1.18a6 6 0 0 0-11.11-2.92l.81.594a5 5 0 0 1 9.3 2.346z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTable;
impl IconShape for VsTable {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.5 2h-12l-.5.5v11l.5.5h12l.5-.5v-11l-.5-.5zM2 3h11v1H2V3zm7 4H6V5h3v2zm0 1v2H6V8h3zM2 5h3v2H2V5zm0 3h3v2H2V8zm0 5v-2h3v2H2zm4 0v-2h3v2H6zm7 0h-3v-2h3v2zm0-3h-3V8h3v2zm-3-3V5h3v2h-3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTag;
impl IconShape for VsTag {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13.2 2H8.017l-.353.146L1 8.81v.707L6.183 14.7h.707l2.215-2.215A4.48 4.48 0 0 0 15.65 9c.027-.166.044-.332.051-.5a4.505 4.505 0 0 0-2-3.74V2.5l-.5-.5zm-.5 2.259A4.504 4.504 0 0 0 11.2 4a.5.5 0 1 0 0 1 3.5 3.5 0 0 1 1.5.338v2.138L8.775 11.4a.506.506 0 0 0-.217.217l-2.022 2.022-4.475-4.476L8.224 3H12.7v1.259zm1 1.792a3.5 3.5 0 0 1 1 2.449 3.438 3.438 0 0 1-.051.5 3.487 3.487 0 0 1-4.793 2.735l3.698-3.698.146-.354V6.051z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTarget;
impl IconShape for VsTarget {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 9C8.55228 9 9 8.55228 9 8C9 7.44772 8.55228 7 8 7C7.44772 7 7 7.44772 7 8C7 8.55228 7.44772 9 8 9Z",
            }
            path {
                d: "M12 8C12 10.2091 10.2091 12 8 12C5.79086 12 4 10.2091 4 8C4 5.79086 5.79086 4 8 4C10.2091 4 12 5.79086 12 8ZM8 11C9.65685 11 11 9.65685 11 8C11 6.34315 9.65685 5 8 5C6.34315 5 5 6.34315 5 8C5 9.65685 6.34315 11 8 11Z",
            }
            path {
                d: "M15 8C15 11.866 11.866 15 8 15C4.13401 15 1 11.866 1 8C1 4.13401 4.13401 1 8 1C11.866 1 15 4.13401 15 8ZM8 14C11.3137 14 14 11.3137 14 8C14 4.68629 11.3137 2 8 2C4.68629 2 2 4.68629 2 8C2 11.3137 4.68629 14 8 14Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTasklist;
impl IconShape for VsTasklist {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3.57 6.699l5.693-4.936L8.585 1 3.273 5.596l-1.51-1.832L1 4.442l1.85 2.214.72.043zM15 5H6.824l2.307-2H15v2zM6 7h9v2H6V7zm9 4H6v2h9v-2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTelescope;
impl IconShape for VsTelescope {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.24 1l.59.24 2.11 4.93-.23.59-3.29 1.41-.59-.24-.17-.41L6.1 9l-.58-.19-.16-.38L2.8 9.49l-.58-.24-.72-1.67.28-.59 2.5-1.06-.18-.41.24-.58L7.9 3.41 7.72 3 8 2.42 11.24 1zM2.5 7.64l.35.85 2.22-.91-.37-.85-2.2.91zm2.74-2.12l1.11 2.45 3-1.28-1.11-2.44-3 1.27zM8.79 3l1.86 4.11 2.29-1.01L11.18 2 8.72 3h.07zM8.5 9.1l3.02 4.9h-1.17l-1.88-3.03v4h-1V9.82L5.58 14h-1.1l1.7-3.9 2.32-1z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTerminalBash;
impl IconShape for VsTerminalBash {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.655 3.56L8.918.75a1.785 1.785 0 0 0-1.82 0L2.363 3.56a1.889 1.889 0 0 0-.921 1.628v5.624a1.889 1.889 0 0 0 .913 1.627l4.736 2.812a1.785 1.785 0 0 0 1.82 0l4.736-2.812a1.888 1.888 0 0 0 .913-1.627V5.188a1.889 1.889 0 0 0-.904-1.627zm-3.669 8.781v.404a.149.149 0 0 1-.07.124l-.239.137c-.038.02-.07 0-.07-.053v-.396a.78.78 0 0 1-.545.053.073.073 0 0 1-.027-.09l.086-.365a.153.153 0 0 1 .071-.096.048.048 0 0 1 .038 0 .662.662 0 0 0 .497-.063.662.662 0 0 0 .37-.567c0-.206-.112-.292-.384-.293-.344 0-.661-.066-.67-.574A1.47 1.47 0 0 1 9.6 9.437V9.03a.147.147 0 0 1 .07-.126l.231-.147c.038-.02.07 0 .07.054v.409a.754.754 0 0 1 .453-.055.073.073 0 0 1 .03.095l-.081.362a.156.156 0 0 1-.065.09.055.055 0 0 1-.035 0 .6.6 0 0 0-.436.072.549.549 0 0 0-.331.486c0 .185.098.242.425.248.438 0 .627.199.632.639a1.568 1.568 0 0 1-.576 1.185zm2.481-.68a.094.094 0 0 1-.036.092l-1.198.727a.034.034 0 0 1-.04.003.035.035 0 0 1-.016-.037v-.31a.086.086 0 0 1 .055-.076l1.179-.706a.035.035 0 0 1 .056.035v.273zm.827-6.914L8.812 7.515c-.559.331-.97.693-.97 1.367v5.52c0 .404.165.662.413.741a1.465 1.465 0 0 1-.248.025c-.264 0-.522-.072-.748-.207L2.522 12.15a1.558 1.558 0 0 1-.75-1.338V5.188a1.558 1.558 0 0 1 .75-1.34l4.738-2.81a1.46 1.46 0 0 1 1.489 0l4.736 2.812a1.548 1.548 0 0 1 .728 1.083c-.154-.334-.508-.427-.92-.185h.002z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTerminalCmd;
impl IconShape for VsTerminalCmd {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.875 7l2.008 5h-.711l-2.008-5h.711zm-5.125.594c-.276 0-.526.041-.75.125a1.542 1.542 0 0 0-.578.375c-.162.166-.287.37-.375.61a2.364 2.364 0 0 0-.133.827c0 .287.04.547.117.781.078.235.196.433.352.594.156.162.346.29.57.383.224.094.48.138.766.133a2.63 2.63 0 0 0 .992-.195l.125.484a1.998 1.998 0 0 1-.492.148 4.381 4.381 0 0 1-.75.07 2.61 2.61 0 0 1-.914-.156 2.207 2.207 0 0 1-.742-.453 1.878 1.878 0 0 1-.485-.742 3.204 3.204 0 0 1-.18-1.023c0-.365.06-.698.18-1 .12-.302.287-.563.5-.782.214-.218.471-.388.774-.507a2.69 2.69 0 0 1 1-.18c.296 0 .536.023.718.07.183.047.315.094.399.14l-.149.493a1.85 1.85 0 0 0-.406-.14 2.386 2.386 0 0 0-.539-.055zM8 8h1v1H8V8zm0 2h1v1H8v-1z",
            }
            path {
                d: "M15.5 1H.5l-.5.5v13l.5.5h15l.5-.5v-13l-.5-.5zM15 14H1V5h14v9zm0-10H1V2h14v2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTerminalDebian;
impl IconShape for VsTerminalDebian {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.084.029a1.276 1.276 0 0 0-.355.05L6.622.065a9.46 9.46 0 0 1 .514-.048c.075-.005.15-.01.224-.017a1.67 1.67 0 0 1-.276.029zm4.127 7.646c.094-.238.172-.436.16-.762l-.133.282c.135-.41.123-.847.112-1.262-.005-.187-.01-.37-.002-.543l-.054-.015c-.048-1.411-1.268-2.911-2.354-3.419-.936-.432-2.376-.506-3.042-.18a.657.657 0 0 1 .212-.085c.107-.031.197-.058.135-.093-.6.06-.778.171-.973.294a1.92 1.92 0 0 1-.635.273c-.11.106.051.063.181.029.129-.035.226-.06-.004.076a1.7 1.7 0 0 1-.303.05c-.26.025-.492.048-.96.532.026.041.11-.009.168-.044.072-.043.106-.063-.054.137C3.07 2.871 1.78 4.31 1.507 4.787l.143.025c-.1.25-.213.461-.313.649-.136.254-.249.464-.273.667a16.97 16.97 0 0 1-.062.635C.907 7.619.79 8.679 1.12 9.06l-.04.406.052.11c.036.079.071.157.12.23l-.093.008c.22.692.338.704.473.717.137.013.291.028.585.757-.084-.028-.17-.06-.293-.226-.015.127.18.508.41.806l-.097.112a.89.89 0 0 0 .27.311c.023.019.045.036.066.055-.372-.203.1.428.371.79.078.104.14.186.159.218l.073-.132c-.01.19.136.433.41.772l.229-.009c.094.186.438.522.647.538l-.139.181c.254.08.321.135.397.195.08.064.17.136.502.253l-.13-.23c.108.095.192.186.273.272.162.176.31.335.62.481.352.123.536.152.74.184.168.026.35.055.649.14a33.82 33.82 0 0 0-.217-.005c-.506-.012-1.056-.025-1.443-.163-3.016-.817-5.776-4.356-5.574-8-.02-.311-.01-.655 0-.961.012-.422.022-.776-.049-.882l.032-.105c.166-.54.365-1.191.742-1.957L.861 3.92v-.002.001c.012.012.106.107.275-.18.04-.09.079-.182.117-.276.08-.19.16-.383.264-.56l.08-.02c.054-.315.533-.744.93-1.1.19-.171.362-.326.46-.443l.02.138C3.541.977 4.414.611 5.074.334c.152-.063.291-.122.414-.176-.107.118.067.082.311.032.15-.03.325-.067.478-.076-.04.023-.082.044-.122.065-.085.045-.17.088-.25.145.26-.062.373-.044.499-.024.109.018.227.036.456.006-.174.025-.384.094-.35.12.245.029.398-.002.537-.03.174-.034.327-.065.61.03L7.625.275c.235.085.409.137.564.183.313.094.55.165 1.067.439a.58.58 0 0 0 .23-.037c.112-.035.218-.069.477.037.014.025.022.046.03.066.03.08.054.143.456.383.056-.022-.097-.162-.22-.274l-.003-.004c1.01.54 2.108 1.692 2.443 2.924-.188-.347-.162-.171-.134.015.018.124.037.253-.006.235.14.377.255.766.325 1.168l-.023-.085c-.102-.368-.3-1.081-.626-1.555-.012.137-.092.122-.165.108-.105-.019-.196-.036-.058.393.081.119.096.074.109.034.015-.047.027-.086.147.164.002.133.034.266.07.414.022.094.046.195.065.306-.034-.006-.07-.07-.106-.13-.045-.076-.087-.147-.117-.101.076.358.201.545.25.572-.009.02-.021.02-.034.021-.027.002-.056.003-.059.167.022.428.102.39.166.361.02-.009.037-.017.051-.01a1.724 1.724 0 0 1-.083.245c-.086.221-.188.48-.106.816a2.356 2.356 0 0 0-.106-.295 5.896 5.896 0 0 1-.046-.117c-.018.151-.01.256-.003.355.013.166.023.312-.094.62.135-.442.12-.841-.007-.649.03.343-.12.642-.254.908-.111.222-.211.42-.184.602l-.161-.222c-.238.344-.22.417-.202.489.015.06.03.12-.105.339.051-.09.041-.112.031-.133-.01-.024-.021-.046.053-.158-.05.003-.17.12-.316.265-.123.121-.265.261-.402.368-1.172.94-2.571 1.062-3.926.556.006-.031-.006-.066-.097-.128-1.148-.88-1.827-1.628-1.591-3.36.068-.051.117-.193.175-.362.09-.263.203-.59.448-.745.245-.541.979-1.04 1.764-1.052.8-.044 1.476.427 1.816.872-.618-.576-1.63-.751-2.493-.324-.882.396-1.405 1.368-1.329 2.336.01-.016.021-.023.03-.03.02-.015.037-.027.048-.108-.027 1.88 2.026 3.258 3.504 2.563l.018.039c.397-.109.497-.205.633-.335.07-.067.148-.142.28-.233a.441.441 0 0 1-.075.085c-.068.067-.143.14-.05.142.166-.043.634-.465.947-.746l.133-.119c.062-.134.051-.177.04-.221-.012-.052-.025-.104.076-.3l.229-.114c.03-.088.062-.168.092-.243zM6.612 10.06a.018.018 0 0 0-.005.016.114.114 0 0 0 .005-.016zm-.005.016c.008.069.269.268.465.369.516.19 1.1.198 1.559.181-.993.415-2.889-.422-3.509-1.532.057.012.168.14.303.297.204.234.462.532.678.605-.213-.17-.377-.387-.53-.61.288.33.637.6 1.019.779a.102.102 0 0 1 .01-.077l.005-.012zM6.752.219a6.612 6.612 0 0 1-.075-.013c.472.014.437.045.283.08.018-.029-.09-.047-.208-.067zM9.63 6.732c.032-.477-.094-.326-.136-.144.019.01.036.059.052.107.028.08.054.158.084.037zm-.211.664a1.68 1.68 0 0 1-.314.703c.006-.061-.038-.074-.083-.086-.092-.026-.183-.052.176-.504a1.113 1.113 0 0 1-.126.242c-.112.184-.21.344.126.133l.033-.06a1.43 1.43 0 0 0 .188-.428zm-1.34 1.247c-.347-.053-.662-.186-.397-.19.221.02.44.02.656-.033a3.544 3.544 0 0 1-.26.223zM6.958.285l-.1.02.094-.008.006-.012zM4.79 8.818l-.038.186c.047.064.092.13.136.195.12.175.237.348.4.483a4.73 4.73 0 0 0-.214-.368c-.08-.13-.169-.272-.285-.496zm.226-.319c.052.108.104.213.185.302l.082.24-.038-.063c-.1-.166-.2-.333-.252-.524l.023.045zm7.474-1.282l-.039.098a4.717 4.717 0 0 1-.462 1.474c.261-.49.43-1.028.501-1.572zM.436 3.426zm.002.022c.008.037.043.028.075.02.06-.015.114-.03-.004.236-.074.052-.119.087-.144.106l-.027.02a.05.05 0 0 1 .008-.017.597.597 0 0 0 .092-.365zM.118 4.76a2.92 2.92 0 0 1-.106.436.588.588 0 0 0-.005-.154c-.013-.105-.025-.197.135-.402a4.009 4.009 0 0 0-.023.12z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTerminalLinux;
impl IconShape for VsTerminalLinux {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.281 11.156a.84.84 0 0 1 .375.297c.084.125.143.276.18.453.02.104.044.2.07.29a1.772 1.772 0 0 0 .219.476c.047.073.11.153.188.242.067.073.127.167.18.281a.793.793 0 0 1 .077.328.49.49 0 0 1-.093.305.944.944 0 0 1-.235.219c-.12.083-.245.156-.375.219-.13.062-.26.127-.39.195a3.624 3.624 0 0 0-.555.328c-.156.115-.313.26-.469.438a2.815 2.815 0 0 1-.625.523 1.471 1.471 0 0 1-.383.172c-.13.036-.26.06-.39.07-.302 0-.552-.052-.75-.156-.198-.104-.37-.294-.516-.57-.042-.079-.083-.128-.125-.149a.774.774 0 0 0-.203-.055L8.67 15c-.26-.02-.525-.031-.796-.031a4.28 4.28 0 0 0-.672.054c-.229.037-.456.081-.68.133-.046.01-.093.05-.14.117a1.7 1.7 0 0 1-.196.227 1.106 1.106 0 0 1-.335.219 1.475 1.475 0 0 1-.555.101c-.172 0-.357-.018-.555-.054a1.82 1.82 0 0 1-.531-.18 3.578 3.578 0 0 0-.953-.328c-.313-.057-.643-.11-.992-.156a3.392 3.392 0 0 1-.344-.063.774.774 0 0 1-.29-.133.705.705 0 0 1-.194-.219.78.78 0 0 1-.079-.351c0-.162.021-.318.063-.469.042-.15.065-.31.07-.476 0-.115-.008-.227-.023-.336a3.53 3.53 0 0 1-.032-.352c0-.265.063-.46.188-.586.125-.125.307-.224.547-.297a.99.99 0 0 0 .297-.148 2.27 2.27 0 0 0 .234-.203 1.86 1.86 0 0 0 .203-.242c.063-.089.133-.178.211-.266a.114.114 0 0 0 .024-.07c0-.063-.003-.123-.008-.18l-.016-.188c0-.354.055-.71.164-1.07.11-.36.253-.71.43-1.055a9.08 9.08 0 0 1 .594-.992c.218-.317.435-.612.648-.883a4.35 4.35 0 0 0 .68-1.203c.15-.416.229-.87.234-1.36 0-.207-.01-.413-.031-.616a6.122 6.122 0 0 1-.031-.625c0-.417.047-.792.14-1.125.094-.334.24-.62.438-.86s.456-.419.773-.539C7.474.075 7.854.01 8.296 0c.527 0 .946.104 1.259.313.312.208.552.481.718.82.167.338.274.716.32 1.133.048.416.074.838.079 1.265v.133c0 .214.002.404.008.57a2.527 2.527 0 0 0 .226.977c.073.161.182.336.328.523.25.329.506.66.766.993.26.333.497.677.71 1.03.214.355.389.725.524 1.11.136.386.206.802.211 1.25a3.3 3.3 0 0 1-.164 1.04zm-6.554-8.14c.072 0 .132.018.18.054a.357.357 0 0 1 .109.149.85.85 0 0 1 .054.187c.01.063.016.128.016.196a.282.282 0 0 1-.024.125.27.27 0 0 1-.07.086l-.094.078a.796.796 0 0 0-.093.093.428.428 0 0 1-.149.141 2.129 2.129 0 0 0-.18.117 1.31 1.31 0 0 0-.156.133.264.264 0 0 0-.07.195c0 .047.023.086.07.117a.704.704 0 0 1 .266.305c.052.12.11.237.172.352.062.114.143.21.242.289.099.078.253.117.46.117h.048c.208-.01.406-.065.594-.164.187-.099.375-.203.562-.313a.633.633 0 0 1 .102-.046.37.37 0 0 0 .101-.055l.57-.445a.926.926 0 0 0 .024-.102 2.75 2.75 0 0 0 .016-.11.236.236 0 0 0-.04-.14.4.4 0 0 0-.093-.094.34.34 0 0 0-.133-.054.909.909 0 0 1-.14-.04 1.083 1.083 0 0 1-.352-.14 1.457 1.457 0 0 0-.344-.156c-.02-.006-.036-.021-.047-.047a.983.983 0 0 1-.031-.094.23.23 0 0 1-.008-.102.126.126 0 0 0-.008-.078c0-.062.005-.127.016-.195a.551.551 0 0 1 .07-.195.417.417 0 0 1 .125-.14.411.411 0 0 1 .203-.056c.162 0 .279.06.352.18.073.12.112.25.117.39a.397.397 0 0 1-.039.18.379.379 0 0 0-.04.172c0 .042.014.07.04.086a.26.26 0 0 0 .102.031c.12 0 .197-.028.234-.085a.533.533 0 0 0 .062-.258c0-.12-.01-.253-.03-.399a1.32 1.32 0 0 0-.126-.406.969.969 0 0 0-.242-.313.574.574 0 0 0-.383-.124c-.27 0-.466.067-.586.203-.12.135-.182.338-.187.609 0 .078.005.156.015.234.01.079.016.157.016.235 0 .026-.003.039-.008.039a.218.218 0 0 1-.047-.016 4.263 4.263 0 0 1-.093-.039.774.774 0 0 0-.118-.039.514.514 0 0 0-.203-.008 1.007 1.007 0 0 1-.125.008c-.073 0-.11-.013-.11-.039 0-.078-.004-.177-.015-.297-.01-.12-.036-.24-.078-.36a.995.995 0 0 0-.156-.296c-.063-.078-.156-.12-.281-.125a.323.323 0 0 0-.227.086.905.905 0 0 0-.164.203.64.64 0 0 0-.086.266 5.4 5.4 0 0 1-.031.25 1.459 1.459 0 0 0 .07.406c.026.083.055.156.086.219.031.062.068.093.11.093.025 0 .06-.018.101-.054.042-.037.063-.07.063-.102 0-.016-.008-.026-.024-.031a.147.147 0 0 0-.047-.008c-.036 0-.068-.018-.094-.055a.468.468 0 0 1-.062-.125 5.144 5.144 0 0 1-.047-.148.564.564 0 0 1 .055-.398c.047-.084.133-.128.258-.133zM5.023 15.18c.125 0 .248-.01.368-.032a.97.97 0 0 0 .336-.125.614.614 0 0 0 .234-.242.943.943 0 0 0 .094-.375.816.816 0 0 0-.047-.273.963.963 0 0 0-.133-.25 2.763 2.763 0 0 0-.203-.281 2.763 2.763 0 0 1-.203-.282 62.93 62.93 0 0 1-.29-.43c-.093-.14-.187-.288-.28-.445a8.124 8.124 0 0 1-.235-.406 2.646 2.646 0 0 0-.266-.398 1.203 1.203 0 0 0-.218-.211.469.469 0 0 0-.29-.094.436.436 0 0 0-.296.11 2.26 2.26 0 0 0-.258.265 3.241 3.241 0 0 1-.297.305c-.11.099-.25.177-.422.234a.744.744 0 0 0-.312.172c-.073.073-.11.185-.11.336 0 .104.008.208.024.312.015.104.026.209.031.313 0 .14-.02.273-.063.398a1.157 1.157 0 0 0-.062.367c0 .141.05.24.148.297.1.058.211.097.336.117.157.027.305.047.446.063.14.016.278.04.414.07.135.032.27.065.406.102.135.036.279.094.43.172.03.015.078.034.14.054l.211.07c.078.027.151.048.219.063a.741.741 0 0 0 .148.024zm2.86-.938c.146 0 .302-.015.469-.047a3.54 3.54 0 0 0 .976-.336 2.59 2.59 0 0 0 .406-.257.222.222 0 0 0 .032-.047.305.305 0 0 0 .023-.063v-.008c.031-.114.057-.24.078-.375a8.63 8.63 0 0 0 .055-.414 8.98 8.98 0 0 1 .055-.414c.02-.135.039-.268.054-.398.021-.14.047-.276.078-.406.032-.13.073-.253.125-.368a1.03 1.03 0 0 1 .211-.304 1.54 1.54 0 0 1 .344-.25v-.016l-.008-.023a.29.29 0 0 1 .047-.149 1.4 1.4 0 0 1 .117-.164.582.582 0 0 1 .149-.133.946.946 0 0 1 .164-.078 9.837 9.837 0 0 0-.102-.375 4.938 4.938 0 0 1-.094-.375 7.126 7.126 0 0 0-.093-.476 2.954 2.954 0 0 0-.11-.36 1.317 1.317 0 0 0-.18-.32c-.077-.104-.174-.23-.288-.375a1.189 1.189 0 0 1-.118-.156.555.555 0 0 1-.046-.196 2.206 2.206 0 0 0-.047-.203 9.48 9.48 0 0 0-.242-.75 2.91 2.91 0 0 0-.172-.383 3.87 3.87 0 0 0-.172-.289c-.052-.078-.107-.117-.164-.117-.125 0-.274.05-.446.149-.171.099-.354.208-.546.328-.193.12-.38.232-.563.336-.182.104-.346.153-.492.148a.7.7 0 0 1-.43-.148 2.236 2.236 0 0 1-.36-.344c-.109-.13-.2-.242-.273-.336-.073-.094-.127-.146-.164-.156-.041 0-.065.031-.07.093a2.56 2.56 0 0 0-.008.211v.133c0 .032-.005.052-.016.063-.057.12-.12.237-.187.351-.068.115-.135.232-.203.352a1.611 1.611 0 0 0-.219.758c0 .078.005.156.016.234.01.078.036.154.078.227l-.016.03a1.31 1.31 0 0 1-.133.157 1.072 1.072 0 0 0-.132.164 2.796 2.796 0 0 0-.407.93c-.078.333-.12.672-.125 1.015 0 .089.006.178.016.266.01.089.016.177.016.266a.526.526 0 0 1-.008.086.525.525 0 0 0-.008.086.75.75 0 0 1 .313.109c.12.068.25.154.39.258.14.104.274.224.399.36.125.135.244.267.359.398.115.13.198.26.25.39.052.13.086.237.101.32a.444.444 0 0 1-.125.329.955.955 0 0 1-.312.203c.089.156.198.289.328.398.13.11.271.198.422.266.151.068.315.117.492.148.177.032.35.047.516.047zm3.133 1.11c.109 0 .216-.016.32-.047a1.65 1.65 0 0 0 .445-.203c.136-.089.26-.198.375-.329a3.07 3.07 0 0 1 .977-.75l.258-.117a2.18 2.18 0 0 0 .257-.133.962.962 0 0 0 .165-.132.256.256 0 0 0 .078-.188.295.295 0 0 0-.024-.117.58.58 0 0 0-.07-.117 5.136 5.136 0 0 1-.203-.305 1.978 1.978 0 0 1-.149-.297l-.125-.312a2.558 2.558 0 0 1-.11-.352.28.28 0 0 0-.054-.101.53.53 0 0 0-.46-.235.533.533 0 0 0-.266.07l-.266.149a7.335 7.335 0 0 1-.281.148.656.656 0 0 1-.297.07.411.411 0 0 1-.258-.077.636.636 0 0 1-.172-.211 2.218 2.218 0 0 1-.117-.258l-.094-.258a1.26 1.26 0 0 1-.14.188.666.666 0 0 0-.125.203c-.068.156-.11.33-.125.523-.026.302-.06.596-.102.883a4.7 4.7 0 0 1-.21.86 1.914 1.914 0 0 0-.063.273 2.88 2.88 0 0 0-.032.289c0 .255.079.466.235.633.156.166.367.25.633.25z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTerminalPowershell;
impl IconShape for VsTerminalPowershell {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.5 1.007l12.999.17.43.501-1.82 12.872-.57.489-13-.17-.43-.502L1.93 1.495l.57-.488zM1.18 13.885l11.998.157 1.68-11.882L2.86 2.003 1.18 13.885zm5.791-3.49l-.14.991 5 .066.14-.99-5-.066zm1.71-2.457l-3.663-2.93-.692.796 2.636 2.112L3.739 9.95l.465.812L8.68 7.938z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTerminalTmux;
impl IconShape for VsTerminalTmux {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13.5 1h-12l-.5.5v13l.5.5h12l.5-.5v-13l-.5-.5zM7 7.5V13H2V2h5v5.5zm6 5.5H8V8h5v5zm0-6H8V2h5v5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTerminalUbuntu;
impl IconShape for VsTerminalUbuntu {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.1365 3.06502C13.1365 4.20551 12.1841 5.13005 11.0093 5.13005C9.83451 5.13005 8.88214 4.20551 8.88214 3.06502C8.88214 1.92454 9.83451 1 11.0093 1C12.1841 1 13.1365 1.92454 13.1365 3.06502Z",
            }
            path {
                d: "M4.25439 7.59961C4.25439 8.74009 3.30201 9.66464 2.12719 9.66464C0.952378 9.66464 0 8.74009 0 7.59961C0 6.45913 0.952378 5.53459 2.12719 5.53459C3.30201 5.53459 4.25439 6.45913 4.25439 7.59961Z",
            }
            path {
                d: "M6.93725 12.9266C5.40046 12.6063 4.11546 11.6538 3.39482 10.3052C2.83046 10.5581 2.18796 10.634 1.57151 10.5244C2.44843 12.6063 4.29779 14.115 6.5639 14.587C7.0588 14.6881 7.57106 14.7387 8.07464 14.7387C7.68394 14.2414 7.46687 13.6346 7.44951 13.0108L7.38203 13.0005C7.23751 12.9786 7.08738 12.9557 6.93725 12.9266Z",
            }
            path {
                d: "M12.6503 12.935C12.6503 14.0755 11.6979 15 10.5231 15C9.34827 15 8.3959 14.0755 8.3959 12.935C8.3959 11.7945 9.34827 10.87 10.5231 10.87C11.6979 10.87 12.6503 11.7945 12.6503 12.935Z",
            }
            path {
                d: "M13.4924 12.1763C14.1523 11.3672 14.6212 10.4063 14.8469 9.38645C15.2376 7.608 14.8729 5.74526 13.8397 4.23653C13.5966 4.80125 13.1799 5.27326 12.6416 5.60198C13.2146 6.65556 13.3883 7.86929 13.1365 9.04088C13.0062 9.61403 12.7892 10.1535 12.4766 10.6423C12.9715 11.03 13.3275 11.5695 13.4924 12.1763Z",
            }
            path {
                d: "M2.03825 4.6288C2.01224 4.63083 1.9872 4.63278 1.96215 4.63278C3.29057 2.35703 5.89529 1.01688 8.56948 1.21916C8.28296 1.57317 8.08326 1.98617 7.98776 2.43289C7.96171 2.5846 7.93566 2.73633 7.92698 2.89647C6.25127 2.93018 4.7058 3.73091 3.74205 5.07107C3.44685 4.89406 3.11692 4.76763 2.76962 4.69178C2.56124 4.64963 2.34418 4.62435 2.12712 4.62435C2.09556 4.62435 2.06634 4.62662 2.03825 4.6288Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTerminal;
impl IconShape for VsTerminal {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 3L3 1.5H21L22.5 3V21L21 22.5H3L1.5 21V3ZM3 3V21H21V3H3Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M7.06078 7.49988L6.00012 8.56054L10.2427 12.8032L6 17.0459L7.06066 18.1066L12 13.1673V12.4391L7.06078 7.49988Z",
            }
            rect {
                height: "1.5",
                width: "6",
                x: "12",
                y: "16.5",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTextSize;
impl IconShape for VsTextSize {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.36 7L1 13h1.34l.51-1.47h2.26L5.64 13H7L4.65 7H3.36zm-.15 3.53l.78-2.14.78 2.14H3.21zM11.82 4h-1.6L7 13h1.56l.75-2.29h3.36l.77 2.29H15l-3.18-9zM9.67 9.5l1.18-3.59c.059-.185.1-.376.12-.57.027.192.064.382.11.57l1.25 3.59H9.67z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsThreeBars;
impl IconShape for VsThreeBars {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14 5H2V3h12v2zm0 4H2V7h12v2zM2 13h12v-2H2v2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsThumbsdownFilled;
impl IconShape for VsThumbsdownFilled {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.9999 7.5V2.5C14.9999 1.673 14.3269 1 13.4999 1H11.9999V9H13.4999C14.3269 9 14.9999 8.327 14.9999 7.5ZM1.42894 9.185C1.80994 9.703 2.39794 10 3.04094 10H6.47394L6.00994 12.32C5.87794 12.981 6.04694 13.658 6.47394 14.18C6.90094 14.701 7.53294 15 8.20594 15C8.64994 15 9.04494 14.702 9.15694 14.307L9.62094 13.021C9.98394 12.016 10.4489 11.056 10.9989 10.145V1H4.60194C3.72194 1 2.95494 1.564 2.69294 2.404L1.13094 7.404C0.938937 8.018 1.04794 8.667 1.42794 9.185H1.42894Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsThumbsdown;
impl IconShape for VsThumbsdown {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.4999 1H4.60292C3.72392 1 2.95692 1.564 2.69392 2.404L1.13192 7.404C0.940924 8.018 1.04892 8.667 1.42992 9.185C1.81092 9.703 2.39892 10 3.04092 10H6.47492L6.01092 12.32C5.87792 12.98 6.04792 13.658 6.47492 14.179C6.90192 14.7 7.53392 14.999 8.20692 14.999C8.65092 14.999 9.04692 14.701 9.15892 14.306L9.62292 13.02C10.1409 11.585 10.8559 10.234 11.7539 8.999H13.5009C14.3279 8.999 15.0009 8.326 15.0009 7.499V2.5C15.0009 1.673 14.3269 1 13.4999 1ZM8.68092 12.682L8.20592 14C7.83292 14 7.48292 13.834 7.24692 13.546C7.01092 13.258 6.91692 12.882 6.98992 12.517L7.57392 9.599C7.60292 9.452 7.56492 9.3 7.46992 9.184C7.37492 9.068 7.23292 9.001 7.08292 9.001H3.03892C2.71792 9.001 2.42392 8.853 2.23292 8.593C2.04292 8.334 1.98892 8.01 2.08492 7.702L3.64692 2.702C3.77792 2.282 4.16192 2 4.60092 2H10.9979V8.336C10.0199 9.668 9.23992 11.13 8.67892 12.682H8.68092ZM13.9999 7.5C13.9999 7.776 13.7749 8 13.4999 8H11.9999V2H13.4999C13.7749 2 13.9999 2.224 13.9999 2.5V7.5Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsThumbsupFilled;
impl IconShape for VsThumbsupFilled {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1 8.5V13.5C1 14.327 1.673 15 2.5 15H4V7H2.5C1.673 7 1 7.673 1 8.5ZM14.571 6.815C14.19 6.297 13.602 6 12.959 6H9.526L9.99 3.68C10.122 3.019 9.953 2.342 9.526 1.82C9.099 1.299 8.467 1 7.794 1C7.35 1 6.955 1.298 6.843 1.693L6.379 2.979C6.016 3.984 5.551 4.944 5.001 5.855V15H11.398C12.278 15 13.045 14.436 13.307 13.596L14.869 8.596C15.061 7.982 14.951 7.333 14.571 6.815Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsThumbsup;
impl IconShape for VsThumbsup {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12.959 6H9.526L9.99 3.68C10.122 3.019 9.953 2.342 9.526 1.82C9.099 1.299 8.467 1 7.794 1C7.35 1 6.955 1.298 6.843 1.693L6.379 2.979C5.861 4.415 5.145 5.766 4.248 7H2.5C1.673 7 1 7.673 1 8.5V13.5C1 14.327 1.673 15 2.5 15H11.397C12.277 15 13.044 14.436 13.306 13.596L14.868 8.596C15.06 7.982 14.951 7.333 14.571 6.815C14.191 6.297 13.602 6 12.959 6ZM2 13.5V8.5C2 8.224 2.224 8 2.5 8H4V14H2.5C2.224 14 2 13.776 2 13.5ZM13.915 8.298L12.353 13.298C12.222 13.717 11.838 14 11.398 14H5.001V7.664C5.979 6.333 6.759 4.872 7.32 3.319L7.795 2.001C8.168 2.001 8.518 2.167 8.754 2.455C8.991 2.744 9.085 3.119 9.011 3.484L8.428 6.402C8.399 6.549 8.437 6.701 8.532 6.817C8.627 6.933 8.769 7 8.919 7H12.962C13.284 7 13.578 7.148 13.768 7.408C13.958 7.668 14.013 7.991 13.917 8.299L13.915 8.298Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTools;
impl IconShape for VsTools {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.773 3.485l-.78-.184-2.108 2.096-1.194-1.216 2.056-2.157-.18-.792a4.42 4.42 0 0 0-1.347-.228 3.64 3.64 0 0 0-1.457.28 3.824 3.824 0 0 0-1.186.84 3.736 3.736 0 0 0-.875 1.265 3.938 3.938 0 0 0 0 2.966 335.341 335.341 0 0 0-6.173 6.234c-.21.275-.31.618-.284.963a1.403 1.403 0 0 0 .464.967c.124.135.272.247.437.328.17.075.353.118.538.127.316-.006.619-.126.854-.337 1.548-1.457 4.514-4.45 6.199-6.204.457.194.948.294 1.444.293a3.736 3.736 0 0 0 2.677-1.133 3.885 3.885 0 0 0 1.111-2.73 4.211 4.211 0 0 0-.196-1.378zM2.933 13.928a.31.31 0 0 1-.135.07.437.437 0 0 1-.149 0 .346.346 0 0 1-.144-.057.336.336 0 0 1-.114-.11c-.14-.143-.271-.415-.14-.568 1.37-1.457 4.191-4.305 5.955-6.046.1.132.21.258.328.376.118.123.245.237.38.341-1.706 1.75-4.488 4.564-5.98 5.994zm11.118-9.065c.002.765-.296 1.5-.832 2.048a2.861 2.861 0 0 1-4.007 0 2.992 2.992 0 0 1-.635-3.137A2.748 2.748 0 0 1 10.14 2.18a2.76 2.76 0 0 1 1.072-.214h.254L9.649 3.839v.696l1.895 1.886h.66l1.847-1.816v.258zM3.24 6.688h1.531l.705.717.678-.674-.665-.678V6.01l.057-1.649-.22-.437-2.86-1.882-.591.066-.831.849-.066.599 1.838 2.918.424.215zm-.945-3.632L4.609 4.58 4.57 5.703H3.494L2.002 3.341l.293-.285zm7.105 6.96l.674-.673 3.106 3.185a1.479 1.479 0 0 1 0 2.039 1.404 1.404 0 0 1-1.549.315 1.31 1.31 0 0 1-.437-.315l-3.142-3.203.679-.678 3.132 3.194a.402.402 0 0 0 .153.105.477.477 0 0 0 .359 0 .403.403 0 0 0 .153-.105.436.436 0 0 0 .1-.153.525.525 0 0 0 .036-.184.547.547 0 0 0-.035-.184.436.436 0 0 0-.1-.153L9.4 10.016z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTrash;
impl IconShape for VsTrash {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 3h3v1h-1v9l-1 1H4l-1-1V4H2V3h3V2a1 1 0 0 1 1-1h3a1 1 0 0 1 1 1v1zM9 2H6v1h3V2zM4 13h7V4H4v9zm2-8H5v7h1V5zm1 0h1v7H7V5zm2 0h1v7H9V5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTriangleDown;
impl IconShape for VsTriangleDown {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 5.56L2.413 5h11.194l.393.54L8.373 11h-.827L2 5.56z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTriangleLeft;
impl IconShape for VsTriangleLeft {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.44 2l.56.413v11.194l-.54.393L5 8.373v-.827L10.44 2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTriangleRight;
impl IconShape for VsTriangleRight {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.56 14L5 13.587V2.393L5.54 2 11 7.627v.827L5.56 14z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTriangleUp;
impl IconShape for VsTriangleUp {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 10.44l-.413.56H2.393L2 10.46 7.627 5h.827L14 10.44z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTwitter;
impl IconShape for VsTwitter {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 3.784a5.63 5.63 0 0 1-.65.803 6.058 6.058 0 0 1-.786.68 5.442 5.442 0 0 1 .014.377c0 .574-.061 1.141-.184 1.702a8.467 8.467 0 0 1-.534 1.627 8.444 8.444 0 0 1-1.264 2.04 7.768 7.768 0 0 1-1.72 1.521 7.835 7.835 0 0 1-2.095.95 8.524 8.524 0 0 1-2.379.329 8.178 8.178 0 0 1-2.293-.325A7.921 7.921 0 0 1 1 12.52a5.762 5.762 0 0 0 4.252-1.19 2.842 2.842 0 0 1-2.273-1.19 2.878 2.878 0 0 1-.407-.8c.091.014.181.026.27.035a2.797 2.797 0 0 0 1.022-.089 2.808 2.808 0 0 1-.926-.362 2.942 2.942 0 0 1-.728-.633 2.839 2.839 0 0 1-.65-1.822v-.033c.402.227.837.348 1.306.362a2.943 2.943 0 0 1-.936-1.04 2.955 2.955 0 0 1-.253-.649 2.945 2.945 0 0 1 .007-1.453c.063-.243.161-.474.294-.693.364.451.77.856 1.216 1.213a8.215 8.215 0 0 0 3.008 1.525 7.965 7.965 0 0 0 1.695.263 2.15 2.15 0 0 1-.058-.325 3.265 3.265 0 0 1-.017-.331c0-.397.075-.77.226-1.118a2.892 2.892 0 0 1 1.528-1.528 2.79 2.79 0 0 1 1.117-.225 2.846 2.846 0 0 1 2.099.909 5.7 5.7 0 0 0 1.818-.698 2.815 2.815 0 0 1-1.258 1.586A5.704 5.704 0 0 0 15 3.785z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTypeHierarchySub;
impl IconShape for VsTypeHierarchySub {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.5 11h-1.729L8.438 6H9.5l.5-.5v-4L9.5 1h-4l-.5.5v4l.5.5h1.062l-3.333 5H1.5l-.5.5v3l.5.5h3l.5-.5v-3l-.5-.5h-.068L7.5 6.4l3.068 4.6H10.5l-.5.5v3l.5.5h3l.5-.5v-3l-.5-.5zM6 5V2h3v3H6zm-2 7v2H2v-2h2zm9 2h-2v-2h2v2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTypeHierarchySuper;
impl IconShape for VsTypeHierarchySuper {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.5 1h-3l-.5.5v3l.5.5h.068L7.5 9.6 4.432 5H4.5l.5-.5v-3L4.5 1h-3l-.5.5v3l.5.5h1.729l3.333 5H5.5l-.5.5v4l.5.5h4l.5-.5v-4l-.5-.5H8.438l3.333-5H13.5l.5-.5v-3l-.5-.5zM2 4V2h2v2H2zm7 7v3H6v-3h3zm4-7h-2V2h2v2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsTypeHierarchy;
impl IconShape for VsTypeHierarchy {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.5 12h-1.793L10 10.293V6.5L9.5 6H8V4h.5l.5-.5v-2L8.5 1h-2l-.5.5v2l.5.5H7v2H5.5l-.5.5v3.793L3.293 12H1.5l-.5.5v2l.5.5h2l.5-.5v-1.793L5.707 11h3.586L11 12.707V14.5l.5.5h2l.5-.5v-2l-.5-.5zM7 2h1v1H7V2zM6 7h3v3H6V7zm-3 7H2v-1h1v1zm10 0h-1v-1h1v1z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsUnfold;
impl IconShape for VsUnfold {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.53 6.51v-4l-1 1-.71-.71L7.65 1h.71l1.84 1.83-.71.7-1-1v3.98h-.96zm0 2.98v4l-1-1-.71.71L7.65 15h.71l1.84-1.83-.71-.7-1 1V9.49h-.96zM13.73 4L14 5.02l-3.68 2.93L14 10.98 13.73 12h-4.2v-1h3L9.55 8.57H6.54L3.45 11h3.08v1H2.27L2 10.98l3.68-2.92L2 5.02 2.27 4h4.26v1H3.45l3 2.42h3.01L12.53 5h-3V4h4.2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsUngroupByRefType;
impl IconShape for VsUngroupByRefType {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.9 1L5 3.1l-.8.7L3 2.6V7H2V2.5L.8 3.8l-.7-.7L2.2 1h.7zM3 13.4V9H2v4.4L.8 12.2l-.7.7L2.2 15h.7L5 12.9l-.7-.7L3 13.4zM8.5 7h-2L6 6.5v-2l.5-.5h2l.5.5v2l-.5.5zM7 6h1V5H7v1zm7.5 1h-3l-.5-.5v-3l.5-.5h3l.5.5v3l-.5.5zM12 6h2V4h-2v2zm-3.5 6h-2l-.5-.5v-2l.5-.5h2l.5.5v2l-.5.5zM7 11h1v-1H7v1zm7.5 2h-3l-.5-.5v-3l.5-.5h3l.5.5v3l-.5.5zM12 12h2v-2h-2v2zm-1-2H9v1h2v-1zm0-5H9v1h2V5z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsUnlock;
impl IconShape for VsUnlock {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 7V5a3 3 0 0 1 5.83-1h1.044A4.002 4.002 0 0 0 4 5v2H3L2 8v6l1 1h10l1-1V8l-1-1H5zm6 1h2v6H3V8h8z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsUnmute;
impl IconShape for VsUnmute {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 4.83h2.79L8.15 1l.85.35v13l-.85.33-3.86-3.85H1.5l-.5-.5v-5l.5-.5zM4.85 10L8 13.14V2.56L4.85 5.68l-.35.15H2v4h2.5l.35.17zM15 7.83a6.97 6.97 0 0 1-1.578 4.428l-.712-.71A5.975 5.975 0 0 0 14 7.83c0-1.4-.48-2.689-1.284-3.71l.712-.71A6.971 6.971 0 0 1 15 7.83zm-2 0a4.978 4.978 0 0 1-1.002 3.004l-.716-.716A3.982 3.982 0 0 0 12 7.83a3.98 3.98 0 0 0-.713-2.28l.716-.716c.626.835.997 1.872.997 2.996zm-2 0c0 .574-.16 1.11-.44 1.566l-.739-.738a1.993 1.993 0 0 0 .005-1.647l.739-.739c.276.454.435.988.435 1.558z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsUnverified;
impl IconShape for VsUnverified {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.67 14.72h.71L10.1 13h2.4l.5-.5v-2.42l1.74-1.72v-.71l-1.71-1.72V3.49l-.5-.49H10.1L8.38 1.29h-.71L6 3H3.53L3 3.5v2.43L1.31 7.65v.71L3 10.08v2.42l.53.5H6l1.67 1.72zM6.16 12H4V9.87l-.12-.35L2.37 8l1.48-1.51.15-.35V4h2.16l.36-.14L8 2.35l1.54 1.51.35.14H12v2.14l.17.35L13.69 8l-1.55 1.52-.14.35V12H9.89l-.38.15L8 13.66l-1.48-1.52-.36-.14zm1.443-5.859a.962.962 0 0 0-.128.291c-.03.109-.05.215-.062.317l-.005.043h-.895l.003-.051c.018-.326.089-.615.212-.864.052-.108.117-.214.193-.318.081-.106.18-.2.294-.28.119-.084.255-.15.409-.2A1.71 1.71 0 0 1 8.165 5c.28 0 .523.046.726.14.2.089.366.21.494.363.127.152.22.326.279.52.058.194.087.394.087.599 0 .191-.032.371-.098.54-.064.164-.143.32-.238.466-.094.143-.197.28-.31.41-.11.129-.211.252-.304.372a2.47 2.47 0 0 0-.23.34.653.653 0 0 0-.088.318v.48h-.888v-.539c0-.168.031-.323.094-.464a2.15 2.15 0 0 1 .24-.401c.096-.127.2-.25.308-.368a4.74 4.74 0 0 0 .299-.356c.093-.12.17-.246.228-.377a.984.984 0 0 0 .09-.421 1.04 1.04 0 0 0-.047-.318v-.001a.638.638 0 0 0-.13-.243.558.558 0 0 0-.216-.158H8.46a.689.689 0 0 0-.294-.059.643.643 0 0 0-.339.083.742.742 0 0 0-.223.215zM8.5 11h-.888v-.888H8.5V11z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsVariableGroup;
impl IconShape for VsVariableGroup {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.387 11.523a.402.402 0 0 1 .593-.367c.058.031.11.065.157.102.047.036.088.07.125.101a.177.177 0 0 0 .117.047c.052 0 .12-.04.203-.117.083-.078.175-.182.273-.313.1-.13.201-.268.305-.414.104-.146.2-.294.29-.445l.226-.39c.062-.11.107-.199.133-.266a15.33 15.33 0 0 0-.133-.524 15.384 15.384 0 0 1-.133-.523 3.72 3.72 0 0 0-.133-.422 1.04 1.04 0 0 0-.187-.313.656.656 0 0 0-.266-.187 1.374 1.374 0 0 0-.375-.07 1.628 1.628 0 0 0-.328.031v-.195L7.69 7a2.345 2.345 0 0 1 .461.734c.052.13.097.263.133.399.037.135.076.283.117.445.078-.115.175-.26.29-.438a4.49 4.49 0 0 1 .398-.523c.15-.172.31-.315.476-.43A1.02 1.02 0 0 1 10.089 7c.13 0 .247.034.351.101.105.068.157.175.157.32 0 .282-.141.423-.422.423a.608.608 0 0 1-.29-.07.608.608 0 0 0-.288-.071c-.1 0-.203.05-.313.148a2.3 2.3 0 0 0-.312.352 9.5 9.5 0 0 0-.485.734l.446 1.852a1.56 1.56 0 0 0 .093.344.669.669 0 0 0 .094.171.184.184 0 0 0 .125.079.37.37 0 0 0 .211-.086 2.14 2.14 0 0 0 .43-.47c.052-.077.093-.15.125-.218l.187.094a2.025 2.025 0 0 1-.219.367 3.775 3.775 0 0 1-.351.422 3.38 3.38 0 0 1-.406.36c-.141.104-.269.153-.383.148a.397.397 0 0 1-.281-.102 1.491 1.491 0 0 1-.204-.234 1.599 1.599 0 0 1-.132-.36 8.263 8.263 0 0 1-.118-.507 34.16 34.16 0 0 1-.101-.532 2.212 2.212 0 0 0-.11-.414l-.203.375a4.489 4.489 0 0 1-.28.453c-.11.157-.222.316-.337.477a2.46 2.46 0 0 1-.375.422c-.135.12-.265.221-.39.305A.66.66 0 0 1 5.91 12a.539.539 0 0 1-.36-.133.454.454 0 0 1-.163-.344zm6.11.477c.28-.36.496-.748.648-1.164a3.87 3.87 0 0 0 .226-1.32c0-.47-.075-.912-.226-1.329A4.57 4.57 0 0 0 11.495 7h.734a3.77 3.77 0 0 1 .922 2.515c0 .474-.073.917-.218 1.329-.146.411-.38.796-.704 1.156h-.734zM3.77 12a3.373 3.373 0 0 1-.704-1.149 3.97 3.97 0 0 1-.218-1.336c0-.953.307-1.791.922-2.515h.726a4.132 4.132 0 0 0-.64 1.18 4.205 4.205 0 0 0-.227 1.335A3.929 3.929 0 0 0 4.496 12H3.77z",
            }
            path {
                d: "M15.5 1H.5l-.5.5v13l.5.5h15l.5-.5v-13l-.5-.5zM15 14H1V5h14v9zm0-10H1V2h14v2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsVerifiedFilled;
impl IconShape for VsVerifiedFilled {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.38 14.72H7.67L6 13H3.53L3 12.5V10.08L1.31 8.36004V7.65004L3 5.93004V3.50004L3.53 3.00004H6L7.67 1.29004H8.38L10.1 3.00004H12.53L13.03 3.49004V5.93004L14.74 7.65004V8.36004L13 10.08V12.5L12.5 13H10.1L8.38 14.72ZM6.73004 10.4799H7.44004L11.21 6.71L10.5 6L7.09004 9.41991L5.71 8.03984L5 8.74984L6.73004 10.4799Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsVerified;
impl IconShape for VsVerified {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.67 14.72h.71L10.1 13h2.4l.5-.5v-2.42l1.74-1.72v-.71l-1.71-1.72V3.49l-.5-.49H10.1L8.38 1.29h-.71L6 3H3.53L3 3.5v2.43L1.31 7.65v.71L3 10.08v2.42l.53.5H6l1.67 1.72zM6.16 12H4V9.87l-.12-.35L2.37 8l1.48-1.51.15-.35V4h2.16l.36-.14L8 2.35l1.54 1.51.35.14H12v2.14l.17.35L13.69 8l-1.55 1.52-.14.35V12H9.89l-.38.15L8 13.66l-1.48-1.52-.36-.14zm.57-1.52h.71l3.77-3.77L10.5 6 7.09 9.42 5.71 8.04 5 8.75l1.73 1.73z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsVersions;
impl IconShape for VsVersions {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8 3L7 4v8l1 1h6l1-1V4l-1-1H8zm6 9H8V4h6v8zM5 9V5h1V4H4.5l-.5.5v7l.5.5H6v-1H5V9zM2 8V6h1V5H1.5l-.5.5v5l.5.5H3v-1H2V8z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsVmActive;
impl IconShape for VsVmActive {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 2h13l.5.5v5.503a5.006 5.006 0 0 0-1-.583V3H2v9h5a5 5 0 0 0 1 3H4v-1h3v-1H1.5l-.5-.5v-10l.5-.5z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M9.778 8.674a4 4 0 1 1 4.444 6.652 4 4 0 0 1-4.444-6.652zm2.13 4.99l2.387-3.182-.8-.6-2.077 2.769-1.301-1.041-.625.78 1.704 1.364.713-.09z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsVmConnect;
impl IconShape for VsVmConnect {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 2h13l.5.5v5.503a5.006 5.006 0 0 0-1-.583V3H2v9h5a5 5 0 0 0 1 3H4v-1h3v-1H1.5l-.5-.5v-10l.5-.5z",
                fill_rule: "evenodd",
            }
            path {
                d: "M12 8a4 4 0 1 0 0 8 4 4 0 0 0 0-8zm0 7a3 3 0 1 1 0-6.001A3 3 0 0 1 12 15z",
            }
            path {
                clip_rule: "evenodd",
                d: "M12.133 11.435l1.436 1.436.431-.431-1.004-1.005L14 10.431l-.431-.43-1.436 1.434zm-1.129 1.067L10 11.498l.431-.431 1.436 1.435-1.436 1.436-.431-.431 1.004-1.005z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsVmOutline;
impl IconShape for VsVmOutline {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 2h13l.5.5v5.503a5.006 5.006 0 0 0-1-.583V3H2v9h5a5 5 0 0 0 1 3H4v-1h3v-1H1.5l-.5-.5v-10l.5-.5z",
                fill_rule: "evenodd",
            }
            path {
                d: "M12 8a4 4 0 1 0 0 8 4 4 0 0 0 0-8zm0 7a3 3 0 1 1 0-6.001A3 3 0 0 1 12 15z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsVmRunning;
impl IconShape for VsVmRunning {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M1.5 2h13l.5.5v5.503a5.006 5.006 0 0 0-1-.583V3H2v9h5a5 5 0 0 0 1 3H4v-1h3v-1H1.5l-.5-.5v-10l.5-.5z",
                fill_rule: "evenodd",
            }
            path {
                d: "M12 8c.367 0 .721.047 1.063.14.34.094.658.23.953.407.294.177.563.385.808.625.245.24.455.509.63.808a4.03 4.03 0 0 1 .405 3.082c-.093.342-.229.66-.406.954a4.382 4.382 0 0 1-.625.808c-.24.245-.509.455-.808.63a4.029 4.029 0 0 1-3.082.405 3.784 3.784 0 0 1-.954-.406 4.382 4.382 0 0 1-.808-.625 3.808 3.808 0 0 1-.63-.808 4.027 4.027 0 0 1-.405-3.082c.093-.342.229-.66.406-.954.177-.294.385-.563.625-.808.24-.245.509-.455.808-.63A4.028 4.028 0 0 1 12 8zm2 3.988L11 10v4l3-2.012z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsVm;
impl IconShape for VsVm {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.5 2h-13l-.5.5v10l.5.5H7v1H4v1h8v-1H9v-1h5.5l.5-.5v-10l-.5-.5zM14 12H2V3h12v9z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsVr;
impl IconShape for VsVr {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 3H12C13.6569 3 15 4.34315 15 6V10C15 11.6569 13.6569 13 12 13H11.6056C11.0133 13 10.4343 12.8247 9.94145 12.4962L9.1094 11.9415C8.4376 11.4936 7.5624 11.4936 6.8906 11.9415L6.05855 12.4962C5.56575 12.8247 4.98672 13 4.39445 13H4C2.34315 13 1 11.6569 1 10V6C1 4.34315 2.34315 3 4 3ZM4 4C2.89543 4 2 4.89543 2 6V10C2 11.1046 2.89543 12 4 12H4.39445C4.7893 12 5.17531 11.8831 5.50385 11.6641L6.3359 11.1094C7.3436 10.4376 8.6564 10.4376 9.6641 11.1094L10.4962 11.6641C10.8247 11.8831 11.2107 12 11.6056 12H12C13.1046 12 14 11.1046 14 10V6C14 4.89543 13.1046 4 12 4H4Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M0 7H1V10H0V7Z",
            }
            path {
                d: "M15 7H16V10H15V7Z",
            }
            path {
                d: "M6.5 8C6.77614 8 7 8.22386 7 8.5C7 8.77614 6.77614 9 6.5 9H4C3.72386 9 3.5 8.77614 3.5 8.5C3.5 8.22386 3.72386 8 4 8H6.5Z",
            }
            path {
                d: "M12 8C12.2761 8 12.5 8.22386 12.5 8.5C12.5 8.77614 12.2761 9 12 9H9.5C9.22386 9 9 8.77614 9 8.5C9 8.22386 9.22386 8 9.5 8H12Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsVscodeInsiders;
impl IconShape for VsVscodeInsiders {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.7851 1.38057L5.56484 5.25075L7.68844 6.87108L10 5.10733V2C10 1.76607 9.91968 1.5509 9.7851 1.38057Z",
            }
            path {
                d: "M10 9.89268L2.58433 4.23442C2.37657 4.0759 2.08597 4.08889 1.89301 4.26532L1.17719 4.91984C1.08223 5.00667 1.02543 5.11898 1.00681 5.23629C0.979153 5.41056 1.03574 5.59585 1.17661 5.72504L9.7851 13.6194C9.91968 13.4491 10 13.2339 10 13V9.89268Z",
            }
            path {
                d: "M10.7532 1.0368C10.9105 1.32251 11 1.65081 11 2V13C11 13.3492 10.9105 13.6775 10.7532 13.9632C10.7906 13.9515 10.8274 13.9369 10.8634 13.9195L13.5399 12.625C13.8211 12.4889 14 12.2028 14 11.8889V3.11109C14 2.79721 13.8212 2.5111 13.5399 2.37507L10.8634 1.08048C10.8274 1.06306 10.7906 1.04852 10.7532 1.0368Z",
            }
            path {
                d: "M1.17661 9.27496L2.37233 8.17842L4.00854 9.6789L2.58433 10.7656C2.37657 10.9241 2.08597 10.9111 1.89301 10.7347L1.17719 10.0802C0.941168 9.86437 0.940898 9.49112 1.17661 9.27496Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsVscode;
impl IconShape for VsVscode {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.8634 13.9195C10.6568 14.0195 10.4233 14.0246 10.2185 13.9444C10.1162 13.9044 10.021 13.843 9.93997 13.7614L4.81616 9.06268L2.58433 10.7656C2.37657 10.9241 2.08597 10.9111 1.89301 10.7347L1.17719 10.0802C0.941168 9.86437 0.940898 9.49112 1.17661 9.27496L3.11213 7.5L1.17661 5.72504C0.940898 5.50888 0.941168 5.13563 1.17719 4.91982L1.89301 4.2653C2.08597 4.08887 2.37657 4.07588 2.58433 4.2344L4.81616 5.93732L9.93997 1.23855C9.97037 1.20797 10.0028 1.18023 10.0368 1.15538C10.2748 0.981429 10.5922 0.949298 10.8634 1.08048L13.5399 2.37507C13.8212 2.5111 14 2.79721 14 3.11109V8H10.752V4.53356L6.86419 7.5L10.752 10.4664V8H14V11.8889C14 12.2028 13.8211 12.4889 13.5399 12.625L10.8634 13.9195Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsWand;
impl IconShape for VsWand {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.38 5h1V4h1V3h-1V2h-1v1h-1v1h1v1zm8 4h-1v1h-1v1h1v1h1v-1h1v-1h-1V9zM14 2V1h-1v1h-1v1h1v1h1V3h1V2h-1zm-2.947 2.442a1.49 1.49 0 0 0-2.12 0l-7.49 7.49a1.49 1.49 0 0 0 0 2.12c.59.59 1.54.59 2.12 0l7.49-7.49c.58-.58.58-1.53 0-2.12zm-8.2 8.9c-.2.2-.51.2-.71 0-.2-.2-.2-.51 0-.71l6.46-6.46.71.71-6.46 6.46zm7.49-7.49l-.32.32-.71-.71.32-.32c.2-.2.51-.2.71 0 .19.2.19.52 0 .71z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsWarning;
impl IconShape for VsWarning {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.56 1h.88l6.54 12.26-.44.74H1.44L1 13.26 7.56 1zM8 2.28L2.28 13H13.7L8 2.28zM8.625 12v-1h-1.25v1h1.25zm-1.25-2V6h1.25v4h-1.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsWatch;
impl IconShape for VsWatch {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.5 9h2V8H8V5.5H7v3l.5.5z",
            }
            path {
                clip_rule: "evenodd",
                d: "M5.5 3.669A4.998 4.998 0 0 0 3 8a4.998 4.998 0 0 0 2.5 4.331V14.5l.5.5h4l.5-.5v-2.169A4.998 4.998 0 0 0 13 8a4.998 4.998 0 0 0-2.5-4.331V1.5L10 1H6l-.5.5v2.169zM12 8a4 4 0 1 1-8 0 4 4 0 0 1 8 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsWhitespace;
impl IconShape for VsWhitespace {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2V1H6.5a3.5 3.5 0 0 0 0 7H8v5H7v1h5v-1h-1V2h1zM8 7H6.5a2.5 2.5 0 1 1 0-5H8v5zm2 6H9V2h1v11z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsWholeWord;
impl IconShape for VsWholeWord {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M0 11H1V13H15V11H16V14H15H1H0V11Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M6.84048 11H5.95963V10.1406H5.93814C5.555 10.7995 4.99104 11.1289 4.24625 11.1289C3.69839 11.1289 3.26871 10.9839 2.95718 10.6938C2.64924 10.4038 2.49527 10.0189 2.49527 9.53906C2.49527 8.51139 3.10041 7.91341 4.3107 7.74512L5.95963 7.51416C5.95963 6.57959 5.58186 6.1123 4.82632 6.1123C4.16389 6.1123 3.56591 6.33789 3.03238 6.78906V5.88672C3.57307 5.54297 4.19612 5.37109 4.90152 5.37109C6.19416 5.37109 6.84048 6.05501 6.84048 7.42285V11ZM5.95963 8.21777L4.63297 8.40039C4.22476 8.45768 3.91682 8.55973 3.70914 8.70654C3.50145 8.84977 3.39761 9.10579 3.39761 9.47461C3.39761 9.74316 3.4925 9.96338 3.68228 10.1353C3.87564 10.3035 4.13166 10.3877 4.45035 10.3877C4.8872 10.3877 5.24706 10.2355 5.52994 9.93115C5.8164 9.62321 5.95963 9.2347 5.95963 8.76562V8.21777Z",
            }
            path {
                d: "M9.3475 10.2051H9.32601V11H8.44515V2.85742H9.32601V6.4668H9.3475C9.78076 5.73633 10.4146 5.37109 11.2489 5.37109C11.9543 5.37109 12.5057 5.61816 12.9032 6.1123C13.3042 6.60286 13.5047 7.26172 13.5047 8.08887C13.5047 9.00911 13.2809 9.74674 12.8333 10.3018C12.3857 10.8532 11.7734 11.1289 10.9964 11.1289C10.2695 11.1289 9.71989 10.821 9.3475 10.2051ZM9.32601 7.98682V8.75488C9.32601 9.20964 9.47282 9.59635 9.76644 9.91504C10.0636 10.2301 10.4396 10.3877 10.8944 10.3877C11.4279 10.3877 11.8451 10.1836 12.1458 9.77539C12.4502 9.36719 12.6024 8.79964 12.6024 8.07275C12.6024 7.46045 12.4609 6.98063 12.1781 6.6333C11.8952 6.28597 11.512 6.1123 11.0286 6.1123C10.5166 6.1123 10.1048 6.29134 9.7933 6.64941C9.48177 7.00391 9.32601 7.44971 9.32601 7.98682Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsWindow;
impl IconShape for VsWindow {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.5 2h-13l-.5.5v11l.5.5h13l.5-.5v-11l-.5-.5zM14 13H2V6h12v7zm0-8H2V3h12v2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsWordWrap;
impl IconShape for VsWordWrap {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3.868 3.449a1.21 1.21 0 0 0-.473-.329c-.274-.111-.623-.15-1.055-.076a3.5 3.5 0 0 0-.71.208c-.082.035-.16.077-.235.125l-.043.03v1.056l.168-.139c.15-.124.326-.225.527-.303.196-.074.4-.113.604-.113.188 0 .33.051.431.157.087.095.137.248.147.456l-.962.144c-.219.03-.41.086-.57.166a1.245 1.245 0 0 0-.398.311c-.103.125-.181.27-.229.426-.097.33-.093.68.011 1.008a1.096 1.096 0 0 0 .638.67c.155.063.328.093.528.093a1.25 1.25 0 0 0 .978-.441v.345h1.007V4.65c0-.255-.03-.484-.089-.681a1.423 1.423 0 0 0-.275-.52zm-.636 1.896v.236c0 .119-.018.231-.055.341a.745.745 0 0 1-.377.447.694.694 0 0 1-.512.027.454.454 0 0 1-.156-.094.389.389 0 0 1-.094-.139.474.474 0 0 1-.035-.186c0-.077.01-.147.024-.212a.33.33 0 0 1 .078-.141.436.436 0 0 1 .161-.109 1.3 1.3 0 0 1 .305-.073l.661-.097zm5.051-1.067a2.253 2.253 0 0 0-.244-.656 1.354 1.354 0 0 0-.436-.459 1.165 1.165 0 0 0-.642-.173 1.136 1.136 0 0 0-.69.223 1.33 1.33 0 0 0-.264.266V1H5.09v6.224h.918v-.281c.123.152.287.266.472.328.098.032.208.047.33.047.255 0 .483-.06.677-.177.192-.115.355-.278.486-.486a2.29 2.29 0 0 0 .293-.718 3.87 3.87 0 0 0 .096-.886 3.714 3.714 0 0 0-.078-.773zm-.86.758c0 .232-.02.439-.06.613-.036.172-.09.315-.159.424a.639.639 0 0 1-.233.237.582.582 0 0 1-.565.014.683.683 0 0 1-.21-.183.925.925 0 0 1-.142-.283A1.187 1.187 0 0 1 6 5.5v-.517c0-.164.02-.314.06-.447.036-.132.087-.242.156-.336a.668.668 0 0 1 .228-.208.584.584 0 0 1 .29-.071.554.554 0 0 1 .496.279c.063.099.108.214.143.354.031.143.05.306.05.482zM2.407 9.9a.913.913 0 0 1 .316-.239c.218-.1.547-.105.766-.018.104.042.204.1.32.184l.33.26V8.945l-.097-.062a1.932 1.932 0 0 0-.905-.215c-.308 0-.593.057-.846.168-.25.11-.467.27-.647.475-.18.21-.318.453-.403.717-.09.272-.137.57-.137.895 0 .289.043.561.13.808.086.249.211.471.373.652.161.185.361.333.597.441.232.104.493.155.778.155.233 0 .434-.028.613-.084.165-.05.322-.123.466-.217l.078-.061v-.889l-.2.095a.4.4 0 0 1-.076.026c-.05.017-.099.035-.128.049-.036.023-.227.09-.227.09-.06.024-.14.043-.218.059a.977.977 0 0 1-.599-.057.827.827 0 0 1-.306-.225 1.088 1.088 0 0 1-.205-.376 1.728 1.728 0 0 1-.076-.529c0-.21.028-.399.083-.56.054-.158.13-.294.22-.4zM14 6h-4V5h4.5l.5.5v6l-.5.5H7.879l2.07 2.071-.706.707-2.89-2.889v-.707l2.89-2.89L9.95 9l-2 2H14V6z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsWorkspaceTrusted;
impl IconShape for VsWorkspaceTrusted {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.069 0c.262 0 .52.017.76.057a4.1 4.1 0 0 1 .697.154c.228.069.451.155.674.263.217.103.44.229.663.366.377.24.748.434 1.126.589a7.537 7.537 0 0 0 2.331.525c.406.029.823.046 1.257.046v4c0 .76-.097 1.48-.291 2.166a8.996 8.996 0 0 1-.789 1.943 10.312 10.312 0 0 1-1.188 1.725 15.091 15.091 0 0 1-1.492 1.532 17.57 17.57 0 0 1-1.703 1.325c-.594.412-1.194.795-1.794 1.143l-.24.143-.24-.143a27.093 27.093 0 0 1-1.806-1.143 15.58 15.58 0 0 1-1.703-1.325 15.082 15.082 0 0 1-1.491-1.532 10.947 10.947 0 0 1-1.194-1.725 9.753 9.753 0 0 1-.789-1.943A7.897 7.897 0 0 1 .571 6V2c.435 0 .852-.017 1.258-.046a8.16 8.16 0 0 0 1.188-.171c.383-.086.766-.2 1.143-.354A6.563 6.563 0 0 0 5.28.846C5.72.56 6.166.349 6.606.21A4.79 4.79 0 0 1 8.069 0zm6.502 2.983a9.566 9.566 0 0 1-2.234-.377 7.96 7.96 0 0 1-2.046-.943A4.263 4.263 0 0 0 9.23 1.16 3.885 3.885 0 0 0 8.074.994a3.99 3.99 0 0 0-1.165.166 3.946 3.946 0 0 0-1.058.503A7.926 7.926 0 0 1 3.8 2.61c-.709.206-1.451.332-2.229.378v3.017c0 .663.086 1.297.258 1.908a8.58 8.58 0 0 0 .72 1.743 9.604 9.604 0 0 0 1.08 1.572c.417.491.862.948 1.342 1.382.48.435.983.835 1.509 1.206.531.372 1.063.709 1.594 1.017a22.397 22.397 0 0 0 1.589-1.017 15.389 15.389 0 0 0 1.514-1.206c.48-.434.926-.891 1.343-1.382a9.596 9.596 0 0 0 1.08-1.572 8.258 8.258 0 0 0 .709-1.743 6.814 6.814 0 0 0 .262-1.908V2.983z",
            }
            path {
                clip_rule: "evenodd",
                d: "M11.797 4.709l-.44-.378-.406.035-4.36 5.148-1.485-2.12-.4-.068-.463.331-.069.4 1.909 2.726.217.12.457.028.234-.102 4.835-5.715-.029-.405z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsWorkspaceUnknown;
impl IconShape for VsWorkspaceUnknown {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.067 0c.263 0 .52.017.76.057a4.1 4.1 0 0 1 .697.154c.229.069.452.155.675.263.217.103.44.229.662.366a7.2 7.2 0 0 0 1.126.589 7.534 7.534 0 0 0 2.332.525c.405.029.822.046 1.257.046v4c0 .76-.097 1.48-.292 2.166a8.996 8.996 0 0 1-.788 1.943 10.306 10.306 0 0 1-1.189 1.725 15.082 15.082 0 0 1-1.491 1.532 17.57 17.57 0 0 1-1.703 1.325c-.594.412-1.194.795-1.794 1.143l-.24.143-.24-.143a27.088 27.088 0 0 1-1.806-1.143 15.579 15.579 0 0 1-1.703-1.325 15.08 15.08 0 0 1-1.491-1.532 10.948 10.948 0 0 1-1.195-1.725 9.753 9.753 0 0 1-.788-1.943A7.897 7.897 0 0 1 .57 6V2c.434 0 .851-.017 1.257-.046a8.16 8.16 0 0 0 1.189-.171c.383-.086.765-.2 1.143-.354a6.563 6.563 0 0 0 1.12-.583C5.719.56 6.164.349 6.604.21A4.79 4.79 0 0 1 8.067 0zm6.503 2.983a9.567 9.567 0 0 1-2.234-.377 7.96 7.96 0 0 1-2.046-.943 4.264 4.264 0 0 0-1.063-.503A3.885 3.885 0 0 0 8.073.994a3.99 3.99 0 0 0-1.166.166 3.946 3.946 0 0 0-1.057.503 7.927 7.927 0 0 1-2.051.948c-.709.206-1.452.332-2.229.378v3.017c0 .663.086 1.297.257 1.908a8.58 8.58 0 0 0 .72 1.743 9.604 9.604 0 0 0 1.08 1.572c.417.491.863.948 1.343 1.382.48.435.983.835 1.509 1.206.531.372 1.062.709 1.594 1.017a22.4 22.4 0 0 0 1.588-1.017 15.384 15.384 0 0 0 1.515-1.206c.48-.434.925-.891 1.343-1.382a9.609 9.609 0 0 0 1.08-1.572 8.269 8.269 0 0 0 .708-1.743 6.814 6.814 0 0 0 .263-1.908V2.983z",
            }
            path {
                clip_rule: "evenodd",
                d: "M9.433 4.72c.171.171.314.377.411.606.103.228.155.48.149.754a1.6 1.6 0 0 1-.114.64 2.24 2.24 0 0 1-.292.48 2.787 2.787 0 0 1-.354.383 4.52 4.52 0 0 0-.337.32 1.421 1.421 0 0 0-.24.32.7.7 0 0 0-.086.348v.36l-.131.138h-.715l-.143-.143V8.57c0-.24.04-.45.12-.634.075-.177.166-.343.28-.486a3.42 3.42 0 0 1 .366-.382c.12-.109.229-.212.332-.32.097-.103.182-.212.245-.326a.707.707 0 0 0 .086-.354.966.966 0 0 0-.074-.36.972.972 0 0 0-.2-.298.94.94 0 0 0-1.32 0 .88.88 0 0 0-.2.298.829.829 0 0 0-.075.36L7 6.21h-.715l-.131-.137c0-.263.046-.514.148-.748.103-.229.24-.435.412-.606.177-.177.383-.32.611-.417a1.883 1.883 0 0 1 1.503 0c.229.103.434.24.606.417zM7.57 9.646l.143-.143h.714l.143.143v.714l-.143.143h-.714l-.143-.143v-.714z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsWorkspaceUntrusted;
impl IconShape for VsWorkspaceUntrusted {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.067 0c.263 0 .52.017.76.057a4.1 4.1 0 0 1 .697.154c.229.069.452.155.675.263.217.103.44.229.662.366a7.2 7.2 0 0 0 1.126.589 7.534 7.534 0 0 0 2.332.525c.405.029.822.046 1.257.046v4c0 .76-.097 1.48-.292 2.166a8.996 8.996 0 0 1-.788 1.943 10.306 10.306 0 0 1-1.189 1.725 15.082 15.082 0 0 1-1.491 1.532 17.57 17.57 0 0 1-1.703 1.325c-.594.412-1.194.795-1.794 1.143l-.24.143-.24-.143a27.088 27.088 0 0 1-1.806-1.143 15.579 15.579 0 0 1-1.703-1.325 15.08 15.08 0 0 1-1.491-1.532 10.948 10.948 0 0 1-1.195-1.725 9.753 9.753 0 0 1-.788-1.943A7.897 7.897 0 0 1 .57 6V2c.434 0 .851-.017 1.257-.046a8.16 8.16 0 0 0 1.189-.171c.383-.086.765-.2 1.143-.354a6.563 6.563 0 0 0 1.12-.583C5.719.56 6.164.349 6.604.21A4.79 4.79 0 0 1 8.067 0zm6.503 2.983a9.567 9.567 0 0 1-2.234-.377 7.96 7.96 0 0 1-2.046-.943 4.264 4.264 0 0 0-1.063-.503A3.885 3.885 0 0 0 8.073.994a3.99 3.99 0 0 0-1.166.166 3.946 3.946 0 0 0-1.057.503 7.927 7.927 0 0 1-2.051.948c-.709.206-1.452.332-2.229.378v3.017c0 .663.086 1.297.257 1.908a8.58 8.58 0 0 0 .72 1.743 9.604 9.604 0 0 0 1.08 1.572c.417.491.863.948 1.343 1.382.48.435.983.835 1.509 1.206.531.372 1.062.709 1.594 1.017a22.4 22.4 0 0 0 1.588-1.017 15.384 15.384 0 0 0 1.515-1.206c.48-.434.925-.891 1.343-1.382a9.609 9.609 0 0 0 1.08-1.572 8.269 8.269 0 0 0 .708-1.743 6.814 6.814 0 0 0 .263-1.908V2.983z",
            }
            path {
                d: "M10.787 5.446l-.4-.406h-.206L8.2 7.023 6.216 5.04h-.2l-.406.406v.2l1.983 1.983L5.61 9.61v.206l.406.4h.2l1.983-1.983 1.982 1.983h.206l.4-.4V9.61L8.804 7.63l1.983-1.983v-.2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsZoomIn;
impl IconShape for VsZoomIn {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.027 6.149a5.52 5.52 0 0 1-1.27 3.908l4.26 4.26-.7.71-4.26-4.27a5.52 5.52 0 1 1 1.97-4.608zm-5.45 4.888a4.51 4.51 0 0 0 3.18-1.32l-.04.02a4.51 4.51 0 0 0 1.36-3.2 4.5 4.5 0 1 0-4.5 4.5zm2.44-4v-1h-2v-2h-1v2h-2v1h2v2h1v-2h2z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VsZoomOut;
impl IconShape for VsZoomOut {
    fn view_box(&self) -> &str {
        "0 0 16 16"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.027 6.149a5.52 5.52 0 0 1-1.27 3.908l4.26 4.26-.7.71-4.26-4.27a5.52 5.52 0 1 1 1.97-4.608zm-5.45 4.888a4.51 4.51 0 0 0 3.18-1.32l-.04.02a4.51 4.51 0 0 0 1.36-3.2 4.5 4.5 0 1 0-4.5 4.5zm-2.54-4.98h5v1h-5v-1z",
                fill_rule: "evenodd",
            }
        }
    }
}
