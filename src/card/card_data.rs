#[derive(serde::Serialize, serde::Deserialize, Debug, PartialOrd, PartialEq)]
pub struct CardData {
    pub word: String,
    pub translated: String,
    pub explanation: String,
    pub pronunciation: String,
    pub sentence: String,
}

#[derive(Debug)]
pub struct CardSet {
    pub name: String,
    cards: Vec<CardData>,
    pub current_card: usize,
    reversed: bool,
    show_pronunciation: bool,
    show_description: bool,
    show_example: bool,
    show_title: bool,
    show_hints: bool,
    zen_mode: bool,
}

const EMPTY_SET: &'static str = "EMPTY SET";

impl CardSet {
    pub fn new(name: &str, cards: Vec<CardData>) -> Self {
        Self {
            name: name.to_string(),
            cards,
            current_card: 0,
            reversed: false,
            show_pronunciation: true,
            show_description: false,
            show_example: false,
            show_title: true,
            show_hints: true,
            zen_mode: false,
        }
    }

    pub fn count_view_weight(&self, x: usize, desc_prefix: usize, example_prefix: usize) -> i32 {
        fn multirow_with_prefix(item: &Option<&str>, x: usize, prefix_len: usize) -> i32 {
            *item
                .map(|text| prefix_len + (*text).len())
                .map(|text_size| if text_size <= x { 1 } else { 2 })
                .get_or_insert(0)
        }
        let mut weight = 0;
        if self.show_title {
            weight -= 1;
        }
        if self.show_hints {
            weight += 1;
        }

        weight += multirow_with_prefix(&self.get_desc(), x, desc_prefix);
        weight += multirow_with_prefix(&self.get_example(), x, example_prefix);

        weight
    }

    pub fn cards_len(&self) -> usize {
        self.cards.len()
    }

    pub fn toggle_show_pronunciation(&mut self) {
        if !self.reversed {
            self.exit_zen_mode_and_turn_optional_elems_off();
            self.show_pronunciation = !self.show_pronunciation;
        }
    }

    pub fn toggle_show_description(&mut self) {
        if !self.reversed {
            self.exit_zen_mode_and_turn_optional_elems_off();
            self.show_description = !self.show_description;
        }
    }

    pub fn toggle_show_example(&mut self) {
        if !self.reversed {
            self.exit_zen_mode_and_turn_optional_elems_off();
            self.show_example = !self.show_example;
        }
    }

    pub fn toggle_show_title(&mut self) {
        self.exit_zen_mode_and_turn_optional_elems_off();
        self.show_title = !self.show_title;
    }

    pub fn toggle_show_hints(&mut self) {
        self.exit_zen_mode_and_turn_optional_elems_off();
        self.show_hints = !self.show_hints;
    }

    fn exit_zen_mode_and_turn_optional_elems_off(&mut self) {
        if self.zen_mode {
            self.show_pronunciation = false;
            self.show_description = false;
            self.show_example = false;
            self.show_title = false;
            self.show_hints = false;
            self.zen_mode = false;
        }
    }

    pub fn toggle_zen_mode(&mut self) {
        self.zen_mode = !self.zen_mode
    }

    pub fn get_main_text(&self) -> &str {
        match self.get_current_card() {
            Some(card) => {
                if self.reversed {
                    &card.translated
                } else {
                    &card.word
                }
            }
            None => EMPTY_SET,
        }
    }

    pub fn get_title(&self) -> Option<&str> {
        if self.show_title && !self.zen_mode {
            Some(&self.name)
        } else {
            None
        }
    }

    pub fn show_hints(&self) -> bool {
        self.show_hints && !self.zen_mode
    }

    pub fn get_pronunciation(&self) -> Option<&str> {
        let card = self.get_current_card()?;
        if self.show_pronunciation && !self.zen_mode && !self.reversed {
            Some(&card.pronunciation)
        } else {
            None
        }
    }

    pub fn get_desc(&self) -> Option<&str> {
        let card = self.get_current_card()?;
        if self.show_description && !self.zen_mode && !self.reversed {
            Some(&card.explanation)
        } else {
            None
        }
    }

    pub fn get_example(&self) -> Option<&str> {
        let card = self.get_current_card()?;
        if self.show_example && !self.zen_mode && !self.reversed {
            Some(&card.sentence)
        } else {
            None
        }
    }

    fn get_current_card(&self) -> Option<&CardData> {
        self.cards.get(self.current_card)
    }

    pub fn reverse_current_card(&mut self) {
        self.reversed = !self.reversed
    }

    pub fn next_card(&mut self) {
        self.current_card = if self.current_card + 1 < self.cards.len() {
            self.current_card + 1
        } else {
            self.current_card
        };
    }

    pub fn prev_card(&mut self) {
        self.current_card = if self.current_card > 0 {
            self.current_card - 1
        } else {
            self.current_card
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card_set(title: &str) -> CardSet {
        CardSet::new(title, vec![gen_card_data(0), gen_card_data(1)])
    }
    fn empty_card_set(title: &str) -> CardSet {
        CardSet::new(title, vec![])
    }

    #[test]
    fn show_default_term_when_empty_set() {
        let set = empty_card_set("empty set");
        assert_eq!(EMPTY_SET, set.get_main_text());
    }

    #[test]
    fn show_default_term_when_reversed_empty_set() {
        let mut set = empty_card_set("empty set");
        set.reversed = true;

        assert_eq!(EMPTY_SET, set.get_main_text());
    }

    #[test]
    fn show_term() {
        let set = card_set("regular set");

        assert_eq!("word0", set.get_main_text());
    }

    #[test]
    fn show_translated_term_when_reversed() {
        let mut set = card_set("regular set");
        set.reversed = true;
        assert_eq!("translated0", set.get_main_text());
    }

    #[test]
    fn show_title_by_default() {
        let set = card_set("set with title");

        assert_eq!(true, set.show_title);
        assert_eq!(Some("set with title"), set.get_title());
    }

    #[test]
    fn show_no_title_when_disabled() {
        let mut set = card_set("set with title");
        assert_eq!(true, set.show_title);

        set.toggle_show_title();

        assert_eq!(None, set.get_title());
    }

    #[test]
    fn show_hints_by_default() {
        let set = card_set("new set");

        assert_eq!(true, set.show_hints);
        assert!(set.show_hints());
    }

    #[test]
    fn show_no_hints_when_disabled() {
        let mut set = card_set("new set");
        assert_eq!(true, set.show_hints);

        set.toggle_show_hints();

        assert!(!set.show_hints());
    }

    #[test]
    fn show_no_hints_when_in_zen() {
        let mut set = card_set("new set");
        assert_eq!(true, set.show_hints);

        set.toggle_zen_mode();

        assert!(!set.show_hints());
    }

    #[test]
    fn show_pronunciation_by_default() {
        let set = card_set("new set");

        assert_eq!(true, set.show_pronunciation);
        assert_eq!(Some("pronunciation0"), set.get_pronunciation());
    }

    #[test]
    fn show_no_pronunciation_when_disabled() {
        let mut set = card_set("new set");
        assert_eq!(true, set.show_pronunciation);

        set.toggle_show_pronunciation();

        assert_eq!(None, set.get_pronunciation());
    }

    #[test]
    fn show_no_pronunciation_when_in_zen() {
        let mut set = card_set("new set");
        assert_eq!(true, set.show_pronunciation);

        set.toggle_zen_mode();

        assert_eq!(None, set.get_pronunciation());
    }

    #[test]
    fn show_no_pronunciation_when_reversed() {
        let mut set = card_set("new set");
        assert_eq!(true, set.show_pronunciation);

        set.toggle_zen_mode();

        assert_eq!(None, set.get_pronunciation());
    }

    #[test]
    fn show_no_pronunciation_when_empty_card_set() {
        let mut set = empty_card_set("empty set");
        set.zen_mode = false;
        set.show_pronunciation = true;

        assert_eq!(None, set.get_pronunciation());
    }

    #[test]
    fn show_no_term_description_by_default() {
        let set = card_set("new set");

        assert_eq!(false, set.show_description);
        assert_eq!(false, set.zen_mode);
        assert_eq!(false, set.reversed);
        assert_eq!(None, set.get_desc());
    }

    #[test]
    fn show_term_description_when_enabled() {
        let mut set = card_set("new set");
        set.show_description = true;
        set.zen_mode = false;
        set.reversed = false;

        assert_eq!(Some("explanation0"), set.get_desc());
    }

    #[test]
    fn show_no_term_description_when_in_zen() {
        let mut set = card_set("new set");
        set.show_description = true;
        set.zen_mode = true;
        set.reversed = false;

        assert_eq!(None, set.get_desc());
    }

    #[test]
    fn show_no_term_description_when_reversed() {
        let mut set = card_set("new set");
        set.show_description = true;
        set.zen_mode = false;
        set.reversed = true;

        assert_eq!(None, set.get_desc());
    }

    #[test]
    fn show_no_term_description_when_empty_card_set() {
        let mut set = empty_card_set("empty set");
        set.show_description = true;
        set.zen_mode = false;
        set.reversed = false;

        assert_eq!(None, set.get_desc());
    }

    #[test]
    fn show_no_example_by_default() {
        let set = card_set("new set");

        assert_eq!(false, set.show_example);
        assert_eq!(false, set.zen_mode);
        assert_eq!(false, set.reversed);
        assert_eq!(None, set.get_example());
    }

    #[test]
    fn show_example_when_enabled() {
        let mut set = card_set("new set");
        set.show_example = true;
        set.zen_mode = false;
        set.reversed = false;

        assert_eq!(Some("sentence0"), set.get_example());
    }

    #[test]
    fn show_no_example_when_in_zen() {
        let mut set = card_set("new set");
        set.show_example = true;
        set.zen_mode = true;
        set.reversed = false;

        assert_eq!(None, set.get_example());
    }

    #[test]
    fn show_no_example_when_reversed() {
        let mut set = card_set("new set");
        set.show_example = true;
        set.zen_mode = false;
        set.reversed = true;

        assert_eq!(None, set.get_example());
    }

    #[test]
    fn show_no_example_when_empty_card_set() {
        let mut set = empty_card_set("empty set");
        set.show_example = true;
        set.zen_mode = false;
        set.reversed = false;

        assert_eq!(None, set.get_example());
    }

    #[test]
    fn prev_goes_to_prev_card() {
        let mut set = card_set("new set");
        set.current_card = 1;

        set.prev_card();

        assert_eq!(0, set.current_card);
    }

    #[test]
    fn cant_prev_when_on_first_card() {
        let mut set = card_set("new set");
        assert_eq!(0, set.current_card);

        set.prev_card();

        assert_eq!(0, set.current_card);
    }

    #[test]
    fn next_goes_to_next_card() {
        let mut set = card_set("new set");
        assert_eq!(0, set.current_card);

        set.next_card();

        assert_eq!(1, set.current_card);
    }

    #[test]
    fn cant_next_when_on_last_card() {
        let mut set = card_set("new set");
        set.current_card = set.cards.len() - 1;

        set.next_card();

        assert_eq!(set.cards.len() - 1, set.current_card);
    }

    #[test]
    fn going_zen_doesnt_change_other_settings_but_displays_only_essential() {
        let mut set = card_set("new set");
        show_everything(&mut set);

        // make sure everything is visible
        set.zen_mode = false;
        assert_all_optional_views_are_visible(&set);

        // turn zen mode on
        set.toggle_zen_mode();
        assert_all_optional_views_are_hidden(&set);

        // go back to previous mode
        set.toggle_zen_mode();
        assert_all_optional_views_are_visible(&set);
    }

    #[test]
    fn when_in_zen_mode_showing_example_exits_zen_and_overrides_other_settings() {
        let mut set = card_set("new set");
        show_everything(&mut set);

        // make sure everything is visible
        set.zen_mode = false;
        assert_all_optional_views_are_visible(&set);

        // turn zen mode on
        set.toggle_zen_mode();
        assert_all_optional_views_are_hidden(&set);

        // turn zen off and show only example
        set.toggle_show_example();
        assert!(set.get_example().is_some());
        assert!(set.get_desc().is_none());
        assert!(set.get_title().is_none());
        assert!(set.get_pronunciation().is_none());
        assert!(!set.show_hints());
    }

    #[test]
    fn when_in_zen_mode_showing_description_exits_zen_and_overrides_other_settings() {
        let mut set = card_set("new set");
        show_everything(&mut set);

        // make sure everything is visible
        set.zen_mode = false;
        assert_all_optional_views_are_visible(&set);

        // turn zen mode on
        set.toggle_zen_mode();
        assert_all_optional_views_are_hidden(&set);

        // turn zen off and show only desc
        set.toggle_show_description();
        assert!(!set.zen_mode);
        assert!(set.get_example().is_none());
        assert!(set.get_desc().is_some());
        assert!(set.get_title().is_none());
        assert!(set.get_pronunciation().is_none());
        assert!(!set.show_hints());
    }

    #[test]
    fn when_in_zen_mode_showing_title_exits_zen_and_overrides_other_settings() {
        let mut set = card_set("new set");
        show_everything(&mut set);

        // make sure everything is visible
        set.zen_mode = false;
        assert_all_optional_views_are_visible(&set);

        // turn zen mode on
        set.toggle_zen_mode();
        assert_all_optional_views_are_hidden(&set);

        // turn zen off and show only title
        set.toggle_show_title();
        assert!(!set.zen_mode);
        assert!(set.get_example().is_none());
        assert!(set.get_desc().is_none());
        assert!(set.get_title().is_some());
        assert!(set.get_pronunciation().is_none());
        assert!(!set.show_hints());
    }

    #[test]
    fn when_in_zen_mode_showing_pronunciation_exits_zen_and_overrides_other_settings() {
        let mut set = card_set("new set");
        show_everything(&mut set);

        // make sure everything is visible
        set.zen_mode = false;
        assert_all_optional_views_are_visible(&set);

        // turn zen mode on
        set.toggle_zen_mode();
        assert_all_optional_views_are_hidden(&set);

        // turn zen off and show only pronunciation
        set.toggle_show_pronunciation();
        assert!(!set.zen_mode);
        assert!(set.get_example().is_none());
        assert!(set.get_desc().is_none());
        assert!(set.get_title().is_none());
        assert!(set.get_pronunciation().is_some());
        assert!(!set.show_hints());
    }

    #[test]
    fn when_in_zen_mode_showing_hints_exits_zen_and_overrides_other_settings() {
        let mut set = card_set("new set");
        show_everything(&mut set);

        // make sure everything is visible
        set.zen_mode = false;
        assert_all_optional_views_are_visible(&set);

        // turn zen mode on
        set.toggle_zen_mode();
        assert_all_optional_views_are_hidden(&set);

        // turn zen off and show only hints
        set.toggle_show_hints();
        assert!(!set.zen_mode);
        assert!(set.get_example().is_none());
        assert!(set.get_desc().is_none());
        assert!(set.get_title().is_none());
        assert!(set.get_pronunciation().is_none());
        assert!(set.show_hints());
    }

    #[test]
    fn if_reversed_dont_toggle_pronunciation() {
        let mut set = card_set("new set");
        set.show_pronunciation = true;
        set.reversed = true;

        set.toggle_show_pronunciation();

        assert_eq!(true, set.show_pronunciation)
    }

    #[test]
    fn if_reversed_dont_toggle_description() {
        let mut set = card_set("new set");
        set.show_description = true;
        set.reversed = true;

        set.toggle_show_description();

        assert_eq!(true, set.show_description)
    }

    #[test]
    fn if_reversed_dont_toggle_example() {
        let mut set = card_set("new set");
        set.show_example = true;
        set.reversed = true;

        set.toggle_show_example();

        assert_eq!(true, set.show_example)
    }

    #[test]
    fn title_tilts_weight_top() {
        let mut set = card_set("test set");
        hide_everything(&mut set);
        set.show_title = true;

        assert_eq!(
            -1,
            set.count_view_weight(usize::max_value(), usize::max_value(), usize::max_value())
        );
    }

    #[test]
    fn hints_tilt_weight_bottom() {
        let mut set = card_set("test set");
        hide_everything(&mut set);
        set.show_hints = true;

        assert_eq!(
            1,
            set.count_view_weight(usize::max_value(), usize::max_value(), usize::max_value())
        );
    }

    #[test]
    fn desc_tilts_weight_bottom() {
        let mut set = card_set("test set");
        hide_everything(&mut set);
        set.show_description = true;

        assert_eq!(
            1,
            set.count_view_weight(usize::max_value(), 0, usize::max_value())
        );
    }

    #[test]
    fn desc_with_prefix_more_than_x_tilts_weight_bottom_by_2() {
        let mut set = card_set("test set");
        hide_everything(&mut set);
        set.show_description = true;

        assert_eq!(2, set.count_view_weight(10, 10, usize::max_value()));
    }

    #[test]
    fn example_tilts_weight_bottom() {
        let mut set = card_set("test set");
        hide_everything(&mut set);
        set.show_example = true;

        assert_eq!(
            1,
            set.count_view_weight(usize::max_value(), usize::max_value(), 0)
        );
    }

    #[test]
    fn example_with_prefix_more_than_x_tilts_weight_bottom_by_2() {
        let mut set = card_set("test set");
        hide_everything(&mut set);
        set.show_example = true;

        assert_eq!(2, set.count_view_weight(10, usize::max_value(), 10));
    }

    fn gen_card_data(nr: i8) -> CardData {
        CardData {
            word: format!("word{}", nr),
            translated: format!("translated{}", nr),
            explanation: format!("explanation{}", nr),
            pronunciation: format!("pronunciation{}", nr),
            sentence: format!("sentence{}", nr),
        }
    }

    fn show_everything(set: &mut CardSet) {
        set.show_example = true;
        set.show_description = true;
        set.show_title = true;
        set.show_hints = true;
        set.show_pronunciation = true;
    }

    fn hide_everything(set: &mut CardSet) {
        set.show_example = false;
        set.show_description = false;
        set.show_title = false;
        set.show_hints = false;
        set.show_pronunciation = false;
    }

    fn assert_all_optional_views_are_visible(set: &CardSet) {
        assert!(set.get_example().is_some());
        assert!(set.get_desc().is_some());
        assert!(set.get_title().is_some());
        assert!(set.get_pronunciation().is_some());
        assert!(set.show_hints());
    }

    fn assert_all_optional_views_are_hidden(set: &CardSet) {
        assert!(set.get_example().is_none());
        assert!(set.get_desc().is_none());
        assert!(set.get_title().is_none());
        assert!(set.get_pronunciation().is_none());
        assert!(!set.show_hints());
    }
}
