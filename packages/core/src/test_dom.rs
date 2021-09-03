//! A DOM for testing - both internal and external code.
use bumpalo::Bump;

use crate::innerlude::*;
use crate::nodes::IntoVNode;

pub struct TestDom {
    bump: Bump,
    scheduler: Scheduler,
}

impl TestDom {
    pub fn new() -> TestDom {
        let bump = Bump::new();
        let mut scheduler = Scheduler::new();
        TestDom { bump, scheduler }
    }

    pub fn new_factory<'a>(&'a self) -> NodeFactory<'a> {
        NodeFactory::new(&self.bump)
    }

    pub fn render<'a, F>(&'a self, lazy_nodes: LazyNodes<'a, F>) -> VNode<'a>
    where
        F: FnOnce(NodeFactory<'a>) -> VNode<'a>,
    {
        lazy_nodes.into_vnode(NodeFactory::new(&self.bump))
    }

    pub fn diff<'a>(&'a self, old: &'a VNode<'a>, new: &'a VNode<'a>) -> Mutations<'a> {
        let mutations = Mutations::new();
        let mut machine = DiffMachine::new(mutations, &self.scheduler.pool);
        machine.stack.push(DiffInstruction::DiffNode { new, old });
        machine.mutations
    }

    pub fn create<'a, F1>(&'a self, left: LazyNodes<'a, F1>) -> Mutations<'a>
    where
        F1: FnOnce(NodeFactory<'a>) -> VNode<'a>,
    {
        let old = self.bump.alloc(self.render(left));

        let mut machine = DiffMachine::new(Mutations::new(), &self.scheduler.pool);

        machine.stack.create_node(old, MountType::Append);

        machine.work(&mut || false);

        machine.mutations
    }

    pub fn lazy_diff<'a, F1, F2>(
        &'a self,
        left: LazyNodes<'a, F1>,
        right: LazyNodes<'a, F2>,
    ) -> (Mutations<'a>, Mutations<'a>)
    where
        F1: FnOnce(NodeFactory<'a>) -> VNode<'a>,
        F2: FnOnce(NodeFactory<'a>) -> VNode<'a>,
    {
        let old = self.bump.alloc(self.render(left));

        let new = self.bump.alloc(self.render(right));

        let mut machine = DiffMachine::new(Mutations::new(), &self.scheduler.pool);

        machine.stack.create_node(old, MountType::Append);

        machine.work(&mut || false);
        let create_edits = machine.mutations;

        let mut machine = DiffMachine::new(Mutations::new(), &self.scheduler.pool);

        machine.stack.push(DiffInstruction::DiffNode { old, new });

        machine.work(&mut || false);

        let edits = machine.mutations;

        (create_edits, edits)
    }
}