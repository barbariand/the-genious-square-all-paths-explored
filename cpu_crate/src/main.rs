pub fn main(
    #[cfg(target_os = "android")] android_app: winit::platform::android::activity::AndroidApp,
) {
    let options: Options = Options::from_args();

    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    return compute::start(&options);
}
