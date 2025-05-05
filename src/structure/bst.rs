use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode = BstNode::new(value);
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        //currentnode.add_parent(Rc::<RefCell<BstNode>>::downgrade(parent));
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new left child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    // tree_insert function
    pub fn tree_insert(&mut self, current_node_link: &BstNodeLink, value: i32){
        let key = self.key.unwrap();
        if value < key{
            if self.left.is_some(){
                let new_left = self.left.clone().unwrap();
                return new_left.borrow_mut().tree_insert(&new_left,value);
            }else{
                self.add_left_child(current_node_link, value);
            }
        }else{
            if self.right.is_some(){
                let new_right = self.right.clone().unwrap();
                return new_right.borrow_mut().tree_insert(&new_right, value);
            }else{
                self.add_right_child(current_node_link, value);
            }
        }
    }

    // transplant function
    fn transplant(&mut self, old_node: Option<BstNodeLink> ,new_node: Option<BstNodeLink>){
        if self.left.is_some() && self.left.clone().unwrap().borrow().key.unwrap() == old_node.clone().unwrap().borrow().key.unwrap(){
            if new_node.is_none(){
                self.left = new_node.clone();
            }else{
                new_node.clone().unwrap().borrow_mut().parent = old_node.clone().unwrap().borrow().parent.clone();
                self.left = new_node.clone();
            }
        }
        if self.right.is_some() && self.right.clone().unwrap().borrow().key.unwrap() == old_node.clone().unwrap().borrow().key.unwrap(){
            if new_node.is_none(){
                self.right = new_node.clone();
            }else{
                new_node.clone().unwrap().borrow_mut().parent = old_node.clone().unwrap().borrow().parent.clone();
                self.right = new_node.clone();
            }
        }
        if new_node.is_some(){
            new_node.unwrap().borrow_mut().parent = old_node.unwrap().borrow().parent.clone();
        }
    }

    // tree_delete function
    pub fn tree_delete(&mut self, target: Option<BstNodeLink>, value: i32){
        if let Some(node) = target{
            // 1st scenario : if the node only have right child
            if node.borrow().right.is_some() && node.borrow().left.is_none() {
                if node.clone().borrow().parent.is_some(){
                    let target_right = node.borrow().right.clone();
                    let node_parent = node.borrow().parent.clone();
                    let strong_parent = BstNode::upgrade_weak_to_strong(node_parent).unwrap();
                    let parent_left = strong_parent.borrow().left.clone();
                    let parent_right = strong_parent.borrow().right.clone();
                    if target_right.clone().unwrap().borrow().key.unwrap() < strong_parent.clone().borrow().key.unwrap() {
                        strong_parent.borrow_mut().transplant(parent_left, target_right);
                    }else{
                        strong_parent.borrow_mut().transplant(parent_right, target_right);
                    }
                }
            }
            // 2nd scenario : if the node only have left child
            else if node.borrow().left.is_some() && node.borrow().right.is_none() {
                if node.clone().borrow().parent.is_some(){
                    let target_left = node.borrow().left.clone();
                    let node_parent = node.borrow().parent.clone();
                    let strong_parent = BstNode::upgrade_weak_to_strong(node_parent).unwrap();
                    let parent_left = strong_parent.borrow().left.clone();
                    let parent_right = strong_parent.borrow().right.clone();
                    if target_left.clone().unwrap().borrow().key.unwrap() < strong_parent.clone().borrow().key.unwrap() {
                        strong_parent.borrow_mut().transplant(parent_left, target_left);
                    }else{
                        strong_parent.borrow_mut().transplant(parent_right, target_left);
                    }
                }
            }
            // 3rd scenario : if the node have two child
            else if node.borrow().left.is_some() && node.borrow().right.is_some(){
                if node.borrow().parent.is_some(){
                    // goes here if the target node have parent
                    let node_parent = node.borrow().parent.clone();
                    let strong_parent = BstNode::upgrade_weak_to_strong(node_parent).unwrap();
                    if node.borrow().key.unwrap() != self.left.clone().unwrap().borrow().key.unwrap() && node.borrow().key.unwrap() != self.right.clone().unwrap().borrow().key.unwrap(){
                        // goes here if the target node is not direct child of root
                        // find the minimum right subtree of target node
                        let y = node.borrow().right.clone().unwrap().borrow().minimum();
                        let y_parent = BstNode::upgrade_weak_to_strong(y.borrow().parent.clone()).unwrap();
                        if BstNode::is_node_match(&node, &y_parent) == false{
                            // replace the right hand of y_parent with the right hand of y node
                            y_parent.borrow_mut().transplant(Some(y.clone()), y.borrow().right.clone());
                            if y.borrow().right.is_some(){
                                y.borrow_mut().right = node.borrow().right.clone();
                                y.borrow_mut().right.as_ref().unwrap().borrow_mut().parent = Some(BstNode::downgrade(&y));
                            }else{
                                y.borrow_mut().right = None;
                            }
                        }
                        // replace the target node with y node
                        strong_parent.borrow_mut().transplant(Some(node.clone()), Some(y.clone()));
                        y.borrow_mut().left = node.borrow().left.clone();
                        y.borrow_mut().left.as_ref().unwrap().borrow_mut().parent = Some(BstNode::downgrade(&y));
                    }else{
                        // goes here if the target node is direct child of root
                        // find the minimum right subtree of the target node
                        let y = node.borrow().right.clone().unwrap().borrow().minimum();
                        let y_parent = BstNode::upgrade_weak_to_strong(y.borrow().parent.clone()).unwrap();
                        if BstNode::is_node_match(&node, &y_parent) == false{
                            y_parent.borrow_mut().transplant(Some(y.clone()), y.borrow().right.clone());
                            if y.borrow().right.is_some(){
                                y.borrow_mut().right = node.borrow().right.clone();
                                y.borrow_mut().right.as_ref().unwrap().borrow_mut().parent = Some(BstNode::downgrade(&y));
                            }else{
                                y.borrow_mut().right = None;
                            }
                        }
                        y.borrow_mut().left = node.borrow().left.clone();
                        y.borrow_mut().left.as_ref().unwrap().borrow_mut().parent = Some(BstNode::downgrade(&y));
                        if y.borrow().key.unwrap() < self.key.unwrap(){
                            self.left = Some(y.clone());
                        }else{
                            self.right = Some(y.clone());
                        }
                    }
                }else{
                    // goes here if the target node is root (doesn't have parent)
                    // find the minimum right subtree of target node
                    let y = node.borrow().right.clone().unwrap().borrow().minimum().clone();
                    let y_parent = BstNode::upgrade_weak_to_strong(y.borrow().parent.clone()).unwrap();
                    y_parent.borrow_mut().transplant(Some(y.clone()), None);
                    self.key = y.borrow().key;
                    self.right = Some(y_parent.clone());
                }
            }
            // 4th scenario : the target node doesn't have either left child or right child
            else if node.borrow().left.is_none() && node.borrow().right.is_none(){
                // condition if the node has parent
                if node.clone().borrow().parent.is_some(){
                    let node_parent = node.borrow().parent.clone();
                    let strong_parent = BstNode::upgrade_weak_to_strong(node_parent).unwrap();
                    let parent_left = strong_parent.borrow().left.clone();
                    let parent_right = strong_parent.borrow().right.clone();
                    if value < strong_parent.borrow().key.unwrap(){
                        strong_parent.borrow_mut().transplant(parent_left, None);
                    }else{
                        strong_parent.borrow_mut().transplant(parent_right, None);
                    }
                }
                // if the node doesn't have parent
                else{
                    self.key = None;
                }
            }
        }
    }

    //search the current tree which node fit the value
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        if let Some(key) = self.key {
            if key == *value {
                return Some(self.get_bst_nodelink_copy());
            }
            if *value < key && self.left.is_some() {
                return self.left.as_ref().unwrap().borrow().tree_search(value);
            } else if self.right.is_some() {
                return self.right.as_ref().unwrap().borrow().tree_search(value);
            }
        }
        //default if current node is NIL
        None
    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */
    pub fn minimum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(left_node) = &self.left {
                return left_node.borrow().minimum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    pub fn maximum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(right_node) = &self.right {
                return right_node.borrow().maximum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

    /**
     * NOTE: Buggy from pull request
     * Find node successor according to the book
     * Should return None, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        // directly check if the node has a right child, otherwise go to the next block
        if let Some(right_node) = &x_node.borrow().right {
            return Some(right_node.borrow().minimum());
        } 
        
        // empty right child case
        else { 
            let mut x_node = x_node;
            let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
            let mut temp: BstNodeLink;

            while let Some(ref exist) = y_node {
                if let Some(ref left_child) = exist.borrow().left {
                    if BstNode::is_node_match(left_child, x_node) {
                        return Some(exist.clone());
                    }
                }

                temp = y_node.unwrap();
                x_node = &temp;
                y_node = BstNode::upgrade_weak_to_strong(temp.borrow().parent.clone());
            }

            None    
        }
    }

    /**
     * Alternate simpler version of tree_successor that made use of is_nil checking
     */
    #[allow(dead_code)]
    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> Option<BstNodeLink>{
        //create a shadow of x_node so it can mutate
        let mut x_node = x_node;
        let right_node = &x_node.borrow().right.clone();
        if BstNode::is_nil(right_node)!=true{
            return Some(right_node.clone().unwrap().borrow().minimum());
        }

        let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
        let y_node_right = &y_node.clone().unwrap().borrow().right.clone();
        let mut y_node2: Rc<RefCell<BstNode>>;
        while BstNode::is_nil(&y_node) && BstNode::is_node_match_option(Some(x_node.clone()), y_node_right.clone()) {
            y_node2 = y_node.clone().unwrap();
            x_node = &y_node2;
            let y_parent = y_node.clone().unwrap().borrow().parent.clone().unwrap();
            y_node = BstNode::upgrade_weak_to_strong(Some(y_parent));
        }

        //in case our sucessor traversal yield root, means self is the highest key
        if BstNode::is_node_match_option(y_node.clone(), Some(BstNode::get_root(&x_node))) {
            return None;
        }

        //default return self / x_node
        return Some(y_node.clone().unwrap())
    }

    /**
     * private function return true if node doesn't has parent nor children nor key
     */
    fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none()
                    || x.borrow().left.is_none()
                    || x.borrow().right.is_none()
                {
                    return true;
                }
                return false;
            }
        }
    }

    //helper function to compare both nodelink
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            Some(x) => Some(x.upgrade().unwrap()),
        }
    }
}
