use shrs::prelude::*;
use shrs_sound::{AudioPlugin, AudioStreamState};

pub fn command_finish_sfx(
    _sh: &Shell,
    ctx: &mut Context,
    _rt: &mut Runtime,
    ac_ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    if let Some(stream) = ctx.state.get::<AudioStreamState>() {
        match ac_ctx.cmd_output.status.success() {
            true => stream.play_sound("success.wav", 0.3)?,
            false => stream.play_sound("error.wav", 0.3)?,
        };
    }
    Ok(())
}

pub fn startup_sfx(
    _sh: &Shell,
    ctx: &mut Context,
    _rt: &mut Runtime,
    _lms_ctx: &StartupCtx,
) -> anyhow::Result<()> {
    if let Some(stream) = ctx.state.get::<AudioStreamState>() {
        stream.play_sound("meow.wav", 0.5)?;
    }

    Ok(())
}

fn main() {
    let mut hooks = Hooks::new();
    hooks.insert(startup_sfx);
    let myshell = ShellBuilder::default()
        .with_plugin(AudioPlugin)
        .with_hooks(hooks)
        .build()
        .unwrap();
    myshell.run().unwrap();
}
