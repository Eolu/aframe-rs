use std::borrow::Cow;
use crate::{*, component::{Position, Rotation}};


#[test]
fn entity_cmp()
{
    use crate::entity::*;

    let child_ent = Entity::new
    (
        vec!(Attribute::new(Cow::Borrowed("child_attr"), Cow::Borrowed("child_val"))), 
        vec!((Cow::Borrowed("child_cmp"), Box::new(component!(Rotation, x: 9.0, y: 55.0, z: 44.0)))), 
        vec!()
    );

    let ent = Entity::new
    (
        vec!(Attribute::new(Cow::Borrowed("test_attr"), Cow::Borrowed("test_val"))), 
        vec!((Cow::Borrowed("test_cmp"), Box::new(component!(Position, x: 1.0, y: 2.0, z: 3.0)))), 
        vec!(child_ent.clone())
    );

    assert_eq!(ent, Entity::new
    (
        vec!(Attribute::new(Cow::Borrowed("test_attr"), Cow::Borrowed("test_val"))), 
        vec!((Cow::Borrowed("test_cmp"), Box::new(component!(Position, x: 1.0, y: 2.0, z: 3.0)))), 
        vec!(child_ent)
    ));
    assert_ne!(ent, Entity::new
    (
        vec!(Attribute::new(Cow::Borrowed("test_attr_2"), Cow::Borrowed("test_val_2"))), 
        vec!((Cow::Borrowed("test_cmp_2"), Box::new(component!(Position, x: 1.0, y: 2.0, z: 3.0)))), 
        vec!()
    ));
}