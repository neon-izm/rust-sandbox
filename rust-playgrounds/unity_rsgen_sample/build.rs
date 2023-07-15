fn main() {
    csbindgen::Builder::default()
        .input_extern_file("./src/lib.rs")
        .csharp_dll_name("unity_rsgen_sample")
        .csharp_class_name("NativeMethods") // optional, default: NativeMethods
        .csharp_namespace("CsBindgen") // optional, default: CsBindgen
        .csharp_class_accessibility("public") // optional, default: internal
        .csharp_entry_point_prefix("") // optional, default: ""
        .csharp_method_prefix("") // optional, default: ""
        .csharp_use_function_pointer(true) // optional, default: true
        .csharp_disable_emit_dll_name(false) // optional, default: false
        .csharp_dll_name_if("UNITY_IOS && !UNITY_EDITOR", "__Internal") // optional, default: ""
         //.generate_csharp_file("../../UnitySimple/Assets/Plugins/NativeMethods.g.cs")
        .generate_csharp_file("../dotnet/NativeMethods.g.cs")
        .unwrap();
}
