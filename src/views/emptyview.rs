use crate::*;

pub struct EmptyView {}

impl View for EmptyView {
    fn print(&self, _id: ViewId, _cx: &mut Context) {
        println!("EmptyView");
    }
    fn draw(&self, _id: ViewId, _cx: &mut Context, _vger: &mut VGER) {}
    fn layout(
        &self,
        _id: ViewId,
        _sz: LocalSize,
        _cx: &mut Context,
        _vger: &mut VGER,
    ) -> LocalSize {
        [0.0, 0.0].into()
    }
    fn hittest(
        &self,
        _id: ViewId,
        _pt: LocalPoint,
        _cx: &mut Context,
        _vger: &mut VGER,
    ) -> Option<ViewId> {
        None
    }
}

impl private::Sealed for EmptyView {}