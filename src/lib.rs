use log::debug;
use neon::prelude::*;
use std::convert::{TryFrom, TryInto};
use std::marker::PhantomData;
use std::sync::Arc;
use voicevox_core::{
    AccelerationMode, AudioQueryModel, InitializeOptions, StyleId, SynthesisOptions, TtsOptions,
    UserDictWord, VoiceModelId,
};

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

#[derive(Default, Debug, Clone)]
struct UserDict {
    dict: voicevox_core::UserDict,
}

impl Finalize for UserDict {}

impl UserDict {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone)]
struct OpenJtalk {
    open_jtalk: Arc<voicevox_core::OpenJtalk>,
}

impl Finalize for OpenJtalk {}

impl OpenJtalk {
    fn new(open_jtalk_dict_dir: String) -> Self {
        let open_jtalk = Arc::new(
            voicevox_core::OpenJtalk::new(open_jtalk_dict_dir).expect("TODO: panic message"),
        );
        OpenJtalk { open_jtalk }
    }
    fn use_user_dict(&self, user_dict: &UserDict) -> Result<(), voicevox_core::Error> {
        self.open_jtalk.use_user_dict(&user_dict.dict)
    }
}

fn openjtalk_constructor(mut cx: FunctionContext) -> JsResult<JsBox<OpenJtalk>> {
    // input
    let open_jtalk_dict_dir = cx.argument::<JsString>(0)?.value(&mut cx);
    // process
    let open_jtalk = OpenJtalk::new(open_jtalk_dict_dir);
    // output
    Ok(cx.boxed(open_jtalk))
}

fn openjtalk_use_user_dict(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    // input
    let this = cx
        .this()
        .downcast_or_throw::<JsBox<OpenJtalk>, _>(&mut cx)?;
    let user_dict = cx.argument::<JsBox<UserDict>>(0)?;
    // process
    this.use_user_dict(&user_dict).expect("TODO: panic message");
    // output
    Ok(cx.undefined())
}

struct Closable<T, C> {
    content: MaybeClosed<T>,
    marker: PhantomData<C>,
}

enum MaybeClosed<T> {
    Open(T),
    Closed,
}

impl<T, C> Closable<T, C> {
    fn new(content: T) -> Self {
        Self {
            content: MaybeClosed::Open(content),
            marker: PhantomData,
        }
    }

    fn get(&self) -> &T {
        match &self.content {
            MaybeClosed::Open(content) => content,
            MaybeClosed::Closed => panic!("TODO: panic message"),
        }
    }

    fn close(&mut self) {
        if matches!(self.content, MaybeClosed::Open(_)) {
            debug!("Closable::close");
        }
        self.content = MaybeClosed::Closed;
    }
}

impl<T, C> Drop for Closable<T, C> {
    fn drop(&mut self) {
        self.close();
    }
}

struct Synthesizer {
    synthesizer: Closable<Arc<voicevox_core::Synthesizer>, Self>,
}

impl Finalize for Synthesizer {}

impl Synthesizer {
    fn new_internal(
        open_jtalk: Handle<JsBox<OpenJtalk>>,
        acceleration_mode: AccelerationMode,
        cpu_num_threads: u16,
    ) -> Self {
        let synthesizer = voicevox_core::Synthesizer::new(
            open_jtalk.open_jtalk.clone(),
            &InitializeOptions {
                acceleration_mode,
                cpu_num_threads,
            },
        )
        .expect("TODO: panic message");
        let synthesizer = Closable::new(Arc::new(synthesizer));
        Self { synthesizer }
    }
}

fn synthesizer_constructor(mut cx: FunctionContext) -> JsResult<JsBox<Synthesizer>> {
    // input
    let open_jtalk = cx.argument::<JsBox<OpenJtalk>>(0)?;
    let acceleration_mode = cx.argument::<JsString>(1)?.value(&mut cx); // タイプ変換を行う
    let acceleration_mode = match acceleration_mode.as_str() {
        "Auto" => AccelerationMode::Auto,
        "Cpu" => AccelerationMode::Cpu,
        "Gpu" => AccelerationMode::Gpu,
        _ => InitializeOptions::default().acceleration_mode,
    };

    let cpu_num_threads = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let cpu_num_threads = match u16::try_from(cpu_num_threads as u64) {
        Ok(num) => num,
        Err(_) => InitializeOptions::default().cpu_num_threads,
    };
    // process
    let instance = Synthesizer::new_internal(open_jtalk, acceleration_mode, cpu_num_threads as u16);
    // output
    Ok(cx.boxed(instance))
}

fn synthesizer_load_model(mut cx: FunctionContext) -> JsResult<JsString> {
    unimplemented!("synthesizer_load_model")
}

fn synthesizer_audio_query(mut cx: FunctionContext) -> JsResult<JsString> {
    unimplemented!("synthesizer_audio_query")
}

fn synthesizer_synthesis(mut cx: FunctionContext) -> JsResult<JsString> {
    unimplemented!("synthesizer_synthesis")
}

fn voice_model_from_path(mut cx: FunctionContext) -> JsResult<JsString> {
    unimplemented!("voice_model_from_path")
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("openjtalkConstructor", openjtalk_constructor)?;
    cx.export_function("openjtalkUseUserDict", openjtalk_use_user_dict)?;
    cx.export_function("synthesizerConstructor", synthesizer_constructor)?;
    cx.export_function("synthesizerLoadModel", synthesizer_load_model)?;
    cx.export_function("synthesizerAudioQuery", synthesizer_audio_query)?;
    cx.export_function("synthesizerSynthesis", synthesizer_synthesis)?;
    cx.export_function("voiceModelFromPath", voice_model_from_path)?;
    Ok(())
}
