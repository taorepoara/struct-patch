#[cfg(any(feature = "box", feature = "option"))]
use crate::Patch;
#[cfg(feature = "box")]
use std::boxed::Box;

#[cfg(feature = "box")]
impl<T, P> Patch<Box<P>> for T
where
    T: Patch<P>,
{
    fn apply(&mut self, patch: Box<P>) {
        self.apply(*patch);
    }

    fn into_patch(self) -> Box<P> {
        Box::new(self.into_patch())
    }

    fn into_patch_by_diff(self, previous_struct: Self) -> Box<P> {
        Box::new(self.into_patch_by_diff(previous_struct))
    }

    fn new_empty_patch() -> Box<P> {
        Box::new(T::new_empty_patch())
    }
}

#[cfg(feature = "option")]
impl<T, P> Patch<Option<P>> for Option<T>
where
    T: Patch<P> + From<P>,
{
    fn apply(&mut self, patch: Option<P>) {
        if let Some(patch) = patch {
            if let Some(self_) = self {
                self_.apply(patch);
            } else {
                *self = Some(patch.into());
            }
        } else {
            *self = None;
        }
    }

    fn into_patch(self) -> Option<P> {
        self.map(|x| x.into_patch())
    }

    fn into_patch_by_diff(self, previous_struct: Self) -> Option<P> {
        match (self, previous_struct) {
            (Some(self_), Some(previous_struct_)) => {
                Some(self_.into_patch_by_diff(previous_struct_))
            }
            (Some(self_), None) => Some(self_.into_patch()),
            (None, _) => None,
        }
    }

    fn new_empty_patch() -> Option<P> {
        Some(T::new_empty_patch())
    }
}

#[cfg(test)]
mod tests {
    use struct_patch::Patch;

    use crate as struct_patch;

    // Tests for Patch<Box<P>> implementation
    #[cfg(feature = "box")]
    mod patch_box {
        use super::*;

        #[test]
        fn test_patch_box_simple() {
            #[derive(Patch, Debug, PartialEq)]
            struct Item {
                field: u32,
                other: String,
            }

            let mut item = Item {
                field: 1,
                other: String::from("hello"),
            };
            let patch = Box::new(ItemPatch {
                field: None,
                other: Some(String::from("bye")),
            });

            item.apply(patch);
            assert_eq!(
                item,
                Item {
                    field: 1,
                    other: String::from("bye")
                }
            );
        }
    }

    // Test for Patch<Option<P>> implementation
    #[cfg(feature = "option")]
    mod patch_option {
        use super::*;

        #[test]
        fn test_patch_option() {
            #[derive(Patch, Debug, PartialEq)]
            struct Item {
                field: u32,
                other: String,
            }

            impl From<ItemPatch> for Item {
                fn from(patch: ItemPatch) -> Self {
                    Item {
                        field: patch.field.unwrap_or_default(),
                        other: patch.other.unwrap_or_default(),
                    }
                }
            }

            let mut item = Some(Item {
                field: 1,
                other: String::from("hello"),
            });
            let patch = Some(ItemPatch {
                field: None,
                other: Some(String::from("bye")),
            });

            item.apply(patch);
            assert_eq!(
                item,
                Some(Item {
                    field: 1,
                    other: String::from("bye")
                })
            );
        }
    }

    // Tests for From<P> for Patch<P> implementation
    #[cfg(feature = "from")]
    mod from_patch {
        use super::*;

        #[test]
        fn test_from_patch() {
            #[derive(Patch, Debug, PartialEq, Default)]
            #[patch(from_patch)]
            struct Item {
                field: u32,
                other: String,
            }

            let patch = ItemPatch {
                field: Some(1),
                other: Some(String::from("hello")),
            };

            let item: Item = patch.into();
            assert_eq!(
                item,
                Item {
                    field: 1,
                    other: String::from("hello")
                }
            );
        }
    }
}
