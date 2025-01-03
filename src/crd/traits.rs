use super::{KeycloakApiObjectOptions, KeycloakApiPatchList};
use kube::{Resource, ResourceExt};
use schemars::JsonSchema;
use serde::Serialize;

pub trait HasApiObject
where
    Self: Resource + Sized,
    Self::Definition: JsonSchema + Serialize,
{
    type Definition;
    fn definition(&self) -> &Self::Definition;

    fn options(&self) -> Option<&KeycloakApiObjectOptions>;

    fn patches(&self) -> Option<&KeycloakApiPatchList>;

    fn prefix() -> &'static str;
}

// sed 's/\$ref.*//; s/^\* spec\.validation\.openAPIV3Schema\.properties\[spec\]\.properties\[definition\]/s/; s/\.properties\[\([^]]*\)\]/.prop("\1")/g; s/\.items\./.array_item()./g; s/\.prop("\([^"]*\)")\.array_items()\.$/.remove("\1");/'
#[macro_export]
macro_rules! api_object_impl {
    ($name:ty, $def:ty, $prefix:literal) => {
        impl $crate::crd::HasApiObject for $name {
            type Definition = $def;
            fn definition(&self) -> &Self::Definition {
                &self.spec.definition
            }

            fn options(
                &self,
            ) -> Option<&$crate::crd::KeycloakApiObjectOptions> {
                self.spec.options.as_ref()
            }

            fn patches(&self) -> Option<&$crate::crd::KeycloakApiPatchList> {
                self.spec.patches.as_ref()
            }

            fn prefix() -> &'static str {
                $prefix
            }
        }

        impl up_impl::HasQuery for $name {
            type Query = $crate::endpoint::query::Query<$name, String>;
        }
    };
}

pub trait HasRoute: Resource + Sized {
    type ParentType;
    type ParentRefType;
    fn id_ident() -> &'static str;
    fn mount_point(&self) -> &'static str;
    fn id_option(&self) -> Option<&str>;

    fn id(&self) -> String {
        if let Some(id) = self.id_option() {
            id.to_string()
        } else {
            let ns = self.namespace();
            let name = self.name_unchecked();
            if let Some(ns) = ns {
                format!("{}_{}", ns, name)
            } else {
                name
            }
        }
    }
}

#[macro_export]
macro_rules! route_impl {
    (<$parent_ty:ty> / |$self_p:ident| $route:block / $id:ident: $self_ty:ident .. $ref:ident: $ref_ty:ty) => {
        impl $crate::crd::HasRoute for $self_ty {
            type ParentType = $parent_ty;
            type ParentRefType = $ref_ty;

            fn id_ident() -> &'static str {
                stringify!($id)
            }

            fn id_option(&self) -> Option<&str> {
                use kube::core::object::HasSpec;
                self.spec().definition.$id.as_deref()
            }

            fn mount_point(&self) -> &'static str {
                let $self_p = self;
                $route
            }
        }

        impl up_impl::HasUp for $self_ty {
            type Up = $parent_ty;
            type UpKey = $ref_ty;

            fn key(&self) -> $ref_ty {
                use kube::core::object::HasSpec;
                self.spec().$ref.clone().into()
            }
        }
    };
    (<$parent_ty:ty> / $route:literal / $id:ident: $self_ty:ident .. $ref:ident: $ref_ty:ty) => {
        $crate::route_impl!(<$parent_ty> / |_x| { $route } / $id: $self_ty .. $ref: $ref_ty);
    };
    ($parent_ty:ident / $route:literal / $id:ident: $self_ty:ident .. $ref:ident: $ref_ty:ty) => {
        $crate::route_impl!(<$parent_ty> / |_x| { $route } / $id: $self_ty .. $ref: $ref_ty);
    };
}

pub use api_object_impl;
pub use route_impl;
