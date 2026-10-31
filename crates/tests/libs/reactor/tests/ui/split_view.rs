use super::*;


#[test]
fn pane_placement_switches_the_pane() -> Result<()> {
    process::run("split-view", |ui| {
        let pane = ui.element("pane")?;
        let content = ui.element("content")?;
        let pane_is_on_the_left = || unsafe {
            Ok(pane.CurrentBoundingRectangle()?.left < content.CurrentBoundingRectangle()?.left)
        };

        ui.wait("pane starts on the left", pane_is_on_the_left, |left| *left)?;
        ui.invoke("toggle")?;
        ui.wait("pane moves to the right", pane_is_on_the_left, |left| !*left)?;
        ui.invoke("toggle")?;
        ui.wait("pane returns to the left", pane_is_on_the_left, |left| *left)?;
        Ok(())

    })
}
