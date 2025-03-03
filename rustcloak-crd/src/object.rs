pub trait KeycloakRestObject {
    type ParentRef;
    type ParentObject;
    type Definition;
    const ID_FIELD: &'static str;
    const PARENT_IDENTIFIER: &'static str;
    const API_PREFIX: &'static str;

    fn id(&self) -> Option<&str>;
    fn mount_path(&self) -> &str;
    fn definition(&self) -> &Self::Definition;
    fn parent_ref(&self) -> &Self::ParentRef;
    fn patches(&self) -> Option<&KeycloakApiPatchList>;
    fn options(&self) -> Option<&KeycloakApiObjectOptions>;
}

macro_rules! impl_object {
    ($api_prefix:literal <$parent_ref:ident: $parent_ref_type:ty => $parent_type:ty> / |$def_v:ident| $mount_path:block / $id_ident:ident for $object_type:ty => $definition_type:ty) => {
        impl_object!($api_prefix <$parent_ref: $parent_ref_type => $parent_type> / |$def_v| $mount_path / ($id_ident => stringify!($id_ident)) for $object_type => $definition_type);
    };
    ($api_prefix:literal <$parent_ref:ident: $parent_ref_type:ty => $parent_type:ty> / |$def_v:ident| $mount_path:block / ($id_ident:ident => $id_lit:expr) for $object_type:ty => $definition_type:ty) => {
        impl $crate::object::KeycloakRestObject for $object_type {
            type ParentRef = $parent_ref_type;
            type ParentObject = $parent_type;
            type Definition = $definition_type;

            const ID_FIELD: &'static str = $id_lit;
            const PARENT_IDENTIFIER: &'static str = stringify!($parent_ref);
            const API_PREFIX: &'static str = $api_prefix;

            fn id(&self) -> Option<&str> {
                self.definition.$id_ident.as_deref()
            }

            fn mount_path(&self) -> &str {
                let $def_v = self;
                $mount_path
            }

            fn definition(&self) -> &Self::Definition {
                &self.definition
            }

            fn parent_ref(&self) -> &Self::ParentRef {
                &self.$parent_ref
            }

            fn patches(&self) -> Option<&$crate::KeycloakApiPatchList> {
                self.patches.as_ref()
            }

            fn options(&self) -> Option<&$crate::KeycloakApiObjectOptions> {
                self.options.as_ref()
            }
        }
    };
}

pub(crate) use impl_object;

use crate::{KeycloakApiObjectOptions, KeycloakApiPatchList};
