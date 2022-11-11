use arboretum::Bst;

#[test]
fn bst_does_contain_item_added_to_it() {
    let bst = &mut Bst::empty();

    bst.insert(5);

    assert_eq!(true, bst.contains(5));
}

#[test]
fn bst_does_not_contain_item_removed_from_it() {
    let bst = &mut Bst::empty();

    bst.insert(5);

    assert_eq!(true, bst.contains(5));

    bst.remove(5);

    assert_eq!(false, bst.contains(5));
}
