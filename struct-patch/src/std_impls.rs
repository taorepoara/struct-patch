use crate::Patch;

// impl<T, P> Patch<Box<P>> for Box<T>
// where
//     T: Patch<P>,
//     Box<P>: From<Box<T>>,
// {
//     fn apply(&mut self, patch: Box<P>) {
//         self.as_mut().apply(*patch);
//     }

//     fn into_patch_by_diff(self, previous_struct: Self) -> Box<P> {
//         Box::new((*self).into_patch_by_diff(*previous_struct))
//     }

//     fn new_empty_patch() -> Box<P> {
//         Box::new(T::new_empty_patch())
//     }
// }

impl<T, P> Patch<Option<P>> for Option<T>
where
    T: Patch<P> + From<P>,
    Option<P>: From<Option<T>>,
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

    // fn into_patch(self) -> Option<P> {
    //     self.map(|x| x.into_patch())
    // }

    fn into_patch_by_diff(self, previous_struct: Self) -> Option<P> {
        match (self, previous_struct) {
            (Some(self_), Some(previous_struct_)) => {
                Some(self_.into_patch_by_diff(previous_struct_))
            }
            (Some(self_), None) => Some(self_.into()),
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

    // Test for Patch<Box<P>> implementation
    // #[test]
    // fn test_patch_box() {
    //     #[derive(Patch, Debug, PartialEq)]
    //     struct Item {
    //         field: u32,
    //         other: String,
    //     }

    //     let mut item = Box::new(Item {
    //         field: 1,
    //         other: String::from("hello"),
    //     });
    //     let patch = Box::new(ItemPatch {
    //         field: None,
    //         other: Some(String::from("bye")),
    //     });

    //     item.apply(patch);
    //     assert_eq!(
    //         item,
    //         Box::new(Item {
    //             field: 1,
    //             other: String::from("bye")
    //         })
    //     );
    // }

    // Test for Patch<Option<P>> implementation
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
