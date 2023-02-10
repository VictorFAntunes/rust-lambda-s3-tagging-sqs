use aws_sdk_s3::model::{Tag, Tagging};
use aws_sdk_s3::output::GetObjectTaggingOutput;

//Trait that covers Tagging and GetObjectTaggingOutput so we can define the trait GenerateTags over both
pub trait TagSet {
    fn tag_set(&self) -> Option<&[Tag]>;
}

impl TagSet for Tagging {
    fn tag_set(&self) -> Option<&[Tag]> {
        self.tag_set()
    }
}

impl TagSet for GetObjectTaggingOutput {
    fn tag_set(&self) -> Option<&[Tag]> {
        self.tag_set()
    }
}

//Extend Tagging functionality by exposing functions that facilitates a particular build patterns
pub trait GenerateTags {
    fn tag_as_true(tag_name: &str) -> Tagging;
    fn tag_as_false(tag_name: &str) -> Tagging;
    fn add_true_tag(&self, tag_name: &str) -> Tagging;
    fn add_false_tag(&self, tag_name: &str) -> Tagging;
    fn replace_with_true_tag(&self, old_tag_name: &str, new_tag_name: &str) -> Tagging;
    fn replace_with_false_tag(&self, old_tag_name: &str, new_tag_name: &str) -> Tagging;
    fn remove_tag(&self, tag_name: &str) -> Tagging;
}

impl<T> GenerateTags for T
where
    T: TagSet,
{
    //Tag the file with a single Tag marked as true
    fn tag_as_true(tag_name: &str) -> Tagging {
        let tag = Tag::builder().key(tag_name).value("true").build();
        Tagging::builder().tag_set(tag).build()
    }
    //Tag the file with a single Tag marked as false
    fn tag_as_false(tag_name: &str) -> Tagging {
        let tag = Tag::builder().key(tag_name).value("false").build();
        Tagging::builder().tag_set(tag).build()
    }

    fn add_true_tag(&self, tag_name: &str) -> Tagging {
        let new_tag = Tag::builder().key(tag_name).value("true").build();
        let tag_set = match self.tag_set() {
            Some(tags) => {
                let mut previous_tags = tags.to_owned();
                previous_tags.retain(|tag| tag.key() != Some(tag_name));
                previous_tags.push(new_tag);
                previous_tags
            }
            None => vec![new_tag],
        };
        Tagging::builder().set_tag_set(tag_set.into()).build()
    }

    //Add another Tag marked as false to the file Tag list
    fn add_false_tag(&self, tag_name: &str) -> Tagging {
        let new_tag = Tag::builder().key(tag_name).value("false").build();
        let tag_set = match self.tag_set() {
            Some(tags) => {
                let mut previous_tags = tags.to_owned();
                previous_tags.retain(|tag| tag.key() != Some(tag_name));
                previous_tags.push(new_tag);
                previous_tags
            }
            None => vec![new_tag],
        };
        Tagging::builder().set_tag_set(tag_set.into()).build()
    }

    //Replace a particular Tag from the file's Tag list with another Tag marked as true
    fn replace_with_true_tag(&self, old_tag_name: &str, new_tag_name: &str) -> Tagging {
        let new_tag = Tag::builder().key(new_tag_name).value("true").build();
        let tag_set = match self.tag_set() {
            Some(tags) => {
                let mut previous_tags = tags.to_owned();
                if previous_tags
                    .iter()
                    .any(|tag| tag.key() == Some(old_tag_name))
                {
                    previous_tags.retain(|tag| tag.key() != Some(old_tag_name));
                    previous_tags.push(new_tag);
                }
                previous_tags
            }
            None => vec![new_tag],
        };
        Tagging::builder().set_tag_set(tag_set.into()).build()
    }

    //Replace a particular Tag from the file's Tag list with another Tag marked as false
    fn replace_with_false_tag(&self, old_tag_name: &str, new_tag_name: &str) -> Tagging {
        let new_tag = Tag::builder().key(new_tag_name).value("false").build();
        let tag_set = match self.tag_set() {
            Some(tags) => {
                let mut previous_tags = tags.to_owned();
                if previous_tags
                    .iter()
                    .any(|tag| tag.key() == Some(old_tag_name))
                {
                    previous_tags.retain(|tag| tag.key() != Some(old_tag_name));
                    previous_tags.push(new_tag);
                }
                previous_tags
            }
            None => vec![new_tag],
        };
        Tagging::builder().set_tag_set(tag_set.into()).build()
    }

    // Remove a particular Tag from the file's Tag list
    fn remove_tag(&self, tag_name: &str) -> Tagging {
        match self.tag_set() {
            Some(tags) => {
                let mut previous_tags = tags.to_owned();
                previous_tags.retain(|tag| tag.key() != Some(tag_name));
                if previous_tags.is_empty() {
                    Tagging::builder().build()
                } else {
                    Tagging::builder().set_tag_set(previous_tags.into()).build()
                }
            }
            None => Tagging::builder().build(),
        }
    }
}

#[cfg(test)]
mod tests_single_tag_pattern {
    use super::*;

    #[test]
    fn test_single_tag_methods_for_tagging() {
        let input: &str = "only_tag";
        let expected_true_output = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("only_tag")
                .value("true")
                .build()]))
            .build();
        let expected_false_output = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("only_tag")
                .value("false")
                .build()]))
            .build();
        assert_eq!(Tagging::tag_as_true(input), expected_true_output);
        assert_eq!(Tagging::tag_as_false(input), expected_false_output);
    }

    #[test]
    fn test_single_tag_methods_for_get_object_tagging_output() {
        let input: &str = "only_tag";
        let expected_true_output = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("only_tag")
                .value("true")
                .build()]))
            .build();
        let expected_false_output = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("only_tag")
                .value("false")
                .build()]))
            .build();
        assert_eq!(
            GetObjectTaggingOutput::tag_as_true(input),
            expected_true_output
        );
        assert_eq!(
            GetObjectTaggingOutput::tag_as_false(input),
            expected_false_output
        );
    }
}

#[cfg(test)]
mod tests_append_tag_pattern {
    use super::*;

    #[test]
    fn test_append_tag_methods_for_tagging() {
        let initial_state = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("initial_tag")
                .value("true")
                .build()]))
            .build();
        let input: &str = "new_tag";
        let expected_true_output = Tagging::builder()
            .set_tag_set(Some(vec![
                Tag::builder().key("initial_tag").value("true").build(),
                Tag::builder().key("new_tag").value("true").build(),
            ]))
            .build();
        let expected_false_output = Tagging::builder()
            .set_tag_set(Some(vec![
                Tag::builder().key("initial_tag").value("true").build(),
                Tag::builder().key("new_tag").value("false").build(),
            ]))
            .build();
        assert_eq!(initial_state.add_true_tag(input), expected_true_output);
        assert_eq!(initial_state.add_false_tag(input), expected_false_output);
    }

    #[test]
    fn test_append_tag_methods_for_tagging_from_empty() {
        let initial_state = Tagging::builder().build();
        let input: &str = "new_tag";
        let expected_true_output = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("new_tag")
                .value("true")
                .build()]))
            .build();
        let expected_false_output = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("new_tag")
                .value("false")
                .build()]))
            .build();
        assert_eq!(initial_state.add_true_tag(input), expected_true_output);
        assert_eq!(initial_state.add_false_tag(input), expected_false_output);
    }

    #[test]
    fn test_append_tag_methods_for_get_object_tagging_output() {
        let initial_state = GetObjectTaggingOutput::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("initial_tag")
                .value("true")
                .build()]))
            .build();
        let input: &str = "new_tag";
        let expected_true_output = Tagging::builder()
            .set_tag_set(Some(vec![
                Tag::builder().key("initial_tag").value("true").build(),
                Tag::builder().key("new_tag").value("true").build(),
            ]))
            .build();
        let expected_false_output = Tagging::builder()
            .set_tag_set(Some(vec![
                Tag::builder().key("initial_tag").value("true").build(),
                Tag::builder().key("new_tag").value("false").build(),
            ]))
            .build();
        assert_eq!(initial_state.add_true_tag(input), expected_true_output);
        assert_eq!(initial_state.add_false_tag(input), expected_false_output);
    }

    #[test]
    fn test_append_tag_methods_for_get_object_tagging_output_from_empty() {
        let initial_state = GetObjectTaggingOutput::builder().build();
        let input: &str = "new_tag";
        let expected_true_output = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("new_tag")
                .value("true")
                .build()]))
            .build();
        let expected_false_output = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("new_tag")
                .value("false")
                .build()]))
            .build();
        assert_eq!(initial_state.add_true_tag(input), expected_true_output);
        assert_eq!(initial_state.add_false_tag(input), expected_false_output);
    }
}

#[cfg(test)]
mod tests_replace_tag_pattern {
    use super::*;

    #[test]
    fn test_replace_tag_methods_for_tagging() {
        let initial_state = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("initial_tag")
                .value("true")
                .build()]))
            .build();
        let expected_true_output = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("new_tag")
                .value("true")
                .build()]))
            .build();
        let expected_false_output = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("new_tag")
                .value("false")
                .build()]))
            .build();
        assert_eq!(
            initial_state.replace_with_true_tag("initial_tag", "new_tag"),
            expected_true_output
        );
        assert_eq!(
            initial_state.replace_with_false_tag("initial_tag", "new_tag"),
            expected_false_output
        );
    }

    #[test]
    fn test_replace_tag_methods_for_tagging_no_matches() {
        let initial_state = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("initial_tag")
                .value("true")
                .build()]))
            .build();
        let expected_true_output = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("initial_tag")
                .value("true")
                .build()]))
            .build();
        let expected_false_output = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("initial_tag")
                .value("true")
                .build()]))
            .build();
        assert_eq!(
            initial_state.replace_with_true_tag("non_existent_tag", "new_tag"),
            expected_true_output
        );
        assert_eq!(
            initial_state.replace_with_false_tag("non_existent_tag", "new_tag"),
            expected_false_output
        );
    }

    #[test]
    fn test_replace_tag_methods_for_tagging_multiple_matches() {
        let initial_state = Tagging::builder()
            .set_tag_set(Some(vec![
                Tag::builder().key("initial_tag").value("true").build(),
                Tag::builder().key("initial_tag").value("false").build(),
                Tag::builder().key("secondary_tag").value("true").build(),
            ]))
            .build();
        let expected_true_output = Tagging::builder()
            .set_tag_set(Some(vec![
                Tag::builder().key("secondary_tag").value("true").build(),
                Tag::builder().key("new_tag").value("true").build(),
            ]))
            .build();
        let expected_false_output = Tagging::builder()
            .set_tag_set(Some(vec![
                Tag::builder().key("secondary_tag").value("true").build(),
                Tag::builder().key("new_tag").value("false").build(),
            ]))
            .build();
        assert_eq!(
            initial_state.replace_with_true_tag("initial_tag", "new_tag"),
            expected_true_output
        );
        assert_eq!(
            initial_state.replace_with_false_tag("initial_tag", "new_tag"),
            expected_false_output
        );
    }

    #[test]
    fn test_replace_tag_methods_for_get_object_tagging_output() {
        let initial_state = GetObjectTaggingOutput::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("initial_tag")
                .value("true")
                .build()]))
            .build();
        let expected_true_output = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("new_tag")
                .value("true")
                .build()]))
            .build();
        let expected_false_output = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("new_tag")
                .value("false")
                .build()]))
            .build();
        assert_eq!(
            initial_state.replace_with_true_tag("initial_tag", "new_tag"),
            expected_true_output
        );
        assert_eq!(
            initial_state.replace_with_false_tag("initial_tag", "new_tag"),
            expected_false_output
        );
    }

    #[test]
    fn test_replace_tag_methods_for_get_object_tagging_output_no_matches() {
        let initial_state = GetObjectTaggingOutput::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("initial_tag")
                .value("true")
                .build()]))
            .build();
        let expected_true_output = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("initial_tag")
                .value("true")
                .build()]))
            .build();
        let expected_false_output = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("initial_tag")
                .value("true")
                .build()]))
            .build();
        assert_eq!(
            initial_state.replace_with_true_tag("non_existent_tag", "new_tag"),
            expected_true_output
        );
        assert_eq!(
            initial_state.replace_with_false_tag("non_existent_tag", "new_tag"),
            expected_false_output
        );
    }

    #[test]
    fn test_replace_tag_methods_for_get_object_tagging_output_multiple_matches() {
        let initial_state = GetObjectTaggingOutput::builder()
            .set_tag_set(Some(vec![
                Tag::builder().key("initial_tag").value("true").build(),
                Tag::builder().key("initial_tag").value("false").build(),
                Tag::builder().key("secondary_tag").value("true").build(),
            ]))
            .build();
        let expected_true_output = Tagging::builder()
            .set_tag_set(Some(vec![
                Tag::builder().key("secondary_tag").value("true").build(),
                Tag::builder().key("new_tag").value("true").build(),
            ]))
            .build();
        let expected_false_output = Tagging::builder()
            .set_tag_set(Some(vec![
                Tag::builder().key("secondary_tag").value("true").build(),
                Tag::builder().key("new_tag").value("false").build(),
            ]))
            .build();
        assert_eq!(
            initial_state.replace_with_true_tag("initial_tag", "new_tag"),
            expected_true_output
        );
        assert_eq!(
            initial_state.replace_with_false_tag("initial_tag", "new_tag"),
            expected_false_output
        );
    }
}

#[cfg(test)]
mod tests_remove_tag_pattern {
    use super::*;

    #[test]
    fn test_remove_tag_method_for_for_tagging() {
        let initial_state = Tagging::builder()
            .set_tag_set(Some(vec![
                Tag::builder().key("initial_tag").value("true").build(),
                Tag::builder().key("secondary_tag").value("true").build(),
            ]))
            .build();
        let expected_output_keep = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("secondary_tag")
                .value("true")
                .build()]))
            .build();
        assert_eq!(
            initial_state.remove_tag("initial_tag"),
            expected_output_keep
        );

        let secondary_state = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("secondary_tag")
                .value("true")
                .build()]))
            .build();
        let expected_output_empty = Tagging::builder().build();
        assert_eq!(
            secondary_state.remove_tag("secondary_tag"),
            expected_output_empty
        );
    }

    #[test]
    fn test_remove_tag_method_for_get_object_tagging_output() {
        let initial_state = GetObjectTaggingOutput::builder()
            .set_tag_set(Some(vec![
                Tag::builder().key("initial_tag").value("true").build(),
                Tag::builder().key("secondary_tag").value("true").build(),
            ]))
            .build();
        let expected_output_keep = Tagging::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("secondary_tag")
                .value("true")
                .build()]))
            .build();
        assert_eq!(
            initial_state.remove_tag("initial_tag"),
            expected_output_keep
        );

        let secondary_state = GetObjectTaggingOutput::builder()
            .set_tag_set(Some(vec![Tag::builder()
                .key("secondary_tag")
                .value("true")
                .build()]))
            .build();
        let expected_output_empty = Tagging::builder().build();
        assert_eq!(
            secondary_state.remove_tag("secondary_tag"),
            expected_output_empty
        );
    }
}
