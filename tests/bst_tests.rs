use arboretum::Bst;

#[test]
fn bst_contains_item_added_to_it() {
    let bst = &mut Bst::empty();

    bst.insert(5);

    assert_eq!(true, bst.contains(5));
}
