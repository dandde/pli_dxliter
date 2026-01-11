use crate::components::editor::script_dropdown::ScriptDropdown;
use dioxus::document::eval;
use dioxus::html::HasFileData;
use dioxus::prelude::*;
use vidyut_lipi::Scheme;

#[derive(Clone, PartialEq, Debug)]
pub enum FileStatus {
    Pending,
    Processing,
    Done,
    Error(String),
}

#[derive(Clone, PartialEq, Debug)]
pub struct FileState {
    pub name: String,
    pub status: FileStatus,
    pub content: Option<String>,
    pub processed_content: Option<String>,
}

#[component]
pub fn FileUpload() -> Element {
    let mut from_script = use_signal(|| "HarvardKyoto".to_string());
    let mut to_script = use_signal(|| "Devanagari".to_string());
    let mut uploaded_files = use_signal(|| Vec::<FileState>::new());
    let mut is_processing = use_signal(|| false);

    let scripts = vec![
        ("Devanagari", "Devanagari"),
        ("Assamese", "Assamese"),
        ("Balinese", "Balinese"),
        ("Bengali", "Bengali"),
        ("Bhaiksuki", "Bhaiksuki"),
        ("Brahmi", "Brahmi"),
        ("Burmese", "Burmese"),
        ("Cham", "Cham"),
        ("Dogra", "Dogra"),
        ("Grantha", "Grantha"),
        ("Gujarati", "Gujarati"),
        ("GunjalaGondi", "Gunjala Gondi"),
        ("Gurmukhi", "Gurmukhi"),
        ("Javanese", "Javanese"),
        ("Kaithi", "Kaithi"),
        ("Kannada", "Kannada"),
        ("Khmer", "Khmer"),
        ("Khudawadi", "Khudawadi"),
        ("Limbu", "Limbu"),
        ("Malayalam", "Malayalam"),
        ("MasaramGondi", "Masaram Gondi"),
        ("MeeteiMayek", "Meetei Mayek"),
        ("Modi", "Modi"),
        ("Nandinagari", "Nandinagari"),
        ("Newa", "Newa (Nepal Bhasa)"),
        ("Odia", "Odia"),
        ("OlChiki", "Ol Chiki"),
        ("Saurashtra", "Saurashtra"),
        ("Sharada", "Sharada"),
        ("Siddham", "Siddham"),
        ("Sinhala", "Sinhala"),
        ("TaiTham", "Tai Tham"),
        ("Takri", "Takri"),
        ("Tamil", "Tamil"),
        ("Telugu", "Telugu"),
        ("Thai", "Thai"),
        ("Tibetan", "Tibetan"),
        ("Tirhuta", "Tirhuta"),
        ("ZanabazarSquare", "Zanabazar Square"),
        ("BarahaSouth", "Baraha (Southern)"),
        ("HarvardKyoto", "Harvard-Kyoto"),
        ("Iast", "IAST"),
        ("Iso15919", "ISO 15919"),
        ("Itrans", "ITRANS"),
        ("Slp1", "SLP1"),
        ("Velthuis", "Velthuis"),
        ("Wx", "WX"),
    ];

    let scripts_data: Vec<(String, String)> = scripts
        .into_iter()
        .map(|(id, name)| (id.to_string(), name.to_string()))
        .collect();

    let convert_files = move |_| async move {
        is_processing.set(true);

        let files_to_process: Vec<usize> = uploaded_files
            .read()
            .iter()
            .enumerate()
            .filter(|(_, f)| f.status != FileStatus::Done)
            .map(|(i, _)| i)
            .collect();

        let from = from_script
            .read()
            .parse::<Scheme>()
            .unwrap_or(Scheme::HarvardKyoto);
        let target = to_script
            .read()
            .parse::<Scheme>()
            .unwrap_or(Scheme::Devanagari);

        for index in files_to_process {
            {
                let mut list = uploaded_files.write();
                list[index].status = FileStatus::Processing;
            }

            let content = uploaded_files.read()[index].content.clone();
            if let Some(text) = content {
                let result = crate::transliterate(&text, from, target, &*to_script.read());
                {
                    let mut list = uploaded_files.write();
                    list[index].processed_content = Some(result);
                    list[index].status = FileStatus::Done;
                }
            } else {
                let mut list = uploaded_files.write();
                list[index].status = FileStatus::Error("No content".to_string());
            }
        }

        is_processing.set(false);
    };

    let trigger_download = move |content: String, filename: String| {
        let safe_content = content
            .replace('\\', "\\\\")
            .replace('`', "\\`")
            .replace('$', "\\$");
        let safe_name = filename.replace('\'', "\\'");

        let js = format!(
            r#"
                (async function() {{
                    const content = `{}`;
                    const filename = '{}';
                    
                    if ('showSaveFilePicker' in window) {{
                        try {{
                            const handle = await window.showSaveFilePicker({{
                                suggestedName: filename,
                                types: [{{
                                    description: 'Text File',
                                    accept: {{ 'text/plain': ['.txt'] }},
                                }}],
                            }});
                            const writable = await handle.createWritable();
                            await writable.write(content);
                            await writable.close();
                            return;
                        }} catch (err) {{
                            if (err.name === 'AbortError') return;
                        }}
                    }}

                    const blob = new Blob([content], {{ type: 'text/plain' }});
                    const url = URL.createObjectURL(blob);
                    const a = document.createElement('a');
                    a.href = url;
                    a.download = filename;
                    document.body.appendChild(a);
                    a.click();
                    setTimeout(() => {{
                        document.body.removeChild(a);
                        URL.revokeObjectURL(url);
                    }}, 1000);
                }})();
            "#,
            safe_content, safe_name
        );
        let _ = eval(&js);
    };

    let save_all = move |_| async move {
        let files = uploaded_files.read();
        for file in files.iter() {
            if let (Some(content), name) = (&file.processed_content, &file.name) {
                trigger_download(content.clone(), name.clone());
            }
        }
    };

    rsx! {
        div { class: "container upload-container",
            h1 { "Batch Transliterator" }
            p { class: "subtitle", "Upload text files for batch transliteration across Indosphere scripts." }

            div { class: "upload-card",
                div {
                    class: "upload-zone",
                    ondragover: move |evt| {
                        evt.stop_propagation();
                        evt.prevent_default();
                    },
                    ondrop: move |evt| {
                        async move {
                            let files = evt.data().files();
                            for file in files {
                                let name = file.name();
                                // Use read_bytes() and convert Bytes to Vec<u8>
                                let content = if let Ok(bytes) = file.read_bytes().await {
                                    String::from_utf8(bytes.to_vec()).ok()
                                } else {
                                    None
                                };
                                uploaded_files.write().push(FileState {
                                    name,
                                    status: FileStatus::Pending,
                                    content,
                                    processed_content: None,
                                });
                            }
                        }
                    },
                    input {
                        r#type: "file",
                        multiple: true,
                        id: "file-input",
                        style: "display: none",
                        onchange: move |evt| {
                            async move {
                                let files = evt.files();
                                for file in files {
                                    let name = file.name();
                                    // Use read_bytes() and convert Bytes to Vec<u8>
                                    let content = if let Ok(bytes) = file.read_bytes().await {
                                        String::from_utf8(bytes.to_vec()).ok()
                                    } else {
                                        None
                                    };
                                    uploaded_files.write().push(FileState {
                                        name,
                                        status: FileStatus::Pending,
                                        content,
                                        processed_content: None,
                                    });
                                }
                            }
                        }
                    }
                    div { class: "upload-placeholder",
                        svg {
                            class: "upload-icon-svg",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.5",
                            path { d: "M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" }
                        }
                        p { "Drag and drop files here, or click to browse" }
                        label {
                            class: "btn-secondary btn-compact",
                            r#for: "file-input",
                            style: "cursor: pointer; display: inline-block;",
                            "Select Files"
                        }
                    }
                }

                if !uploaded_files.read().is_empty() {
                    div { class: "file-list",
                        h3 { "Selected Files" }
                        ul {
                            for (index, file) in uploaded_files.read().iter().enumerate() {
                                li {
                                    key: "{index}",
                                    class: "file-item",
                                    div { class: "file-info",
                                        span { class: "file-status-icon",
                                            match file.status {
                                                FileStatus::Pending => "⌛",
                                                FileStatus::Processing => "🔄",
                                                FileStatus::Done => "✅",
                                                FileStatus::Error(_) => "❌",
                                            }
                                        }
                                        input {
                                            class: "file-name-input",
                                            value: "{file.name}",
                                            oninput: move |evt| {
                                                uploaded_files.write()[index].name = evt.value();
                                            }
                                        }
                                    }
                                    div { class: "file-actions",
                                        if let FileStatus::Done = file.status {
                                            button {
                                                class: "btn-save-individual",
                                                title: "Save As",
                                                onclick: move |_| {
                                                    let files = uploaded_files.read();
                                                    if let Some(content) = &files[index].processed_content {
                                                        trigger_download(content.clone(), files[index].name.clone());
                                                    }
                                                },
                                                svg {
                                                    class: "save-icon-svg",
                                                    view_box: "0 0 24 24",
                                                    fill: "none",
                                                    stroke: "currentColor",
                                                    stroke_width: "2",
                                                    path { d: "M19 21H5a2 2 0 01-2-2V5a2 2 0 012-2h11l5 5v11a2 2 0 01-2 2z" }
                                                    polyline { points: "17 21 17 13 7 13 7 21" }
                                                    polyline { points: "7 3 7 8 15 8" }
                                                }
                                            }
                                        }
                                        button {
                                            class: "btn-remove",
                                            disabled: is_processing(),
                                            onclick: move |_| {
                                                uploaded_files.write().remove(index);
                                            },
                                            "✕"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "settings-row",
                    div { class: "script-picker",
                        label { "Source Script" }
                        ScriptDropdown {
                            label: "Source",
                            current_script: from_script.read().clone(),
                            scripts: scripts_data.clone(),
                            on_select: move |id| from_script.set(id),
                        }
                    }
                    div { class: "script-picker",
                        label { "Target Script" }
                        ScriptDropdown {
                            label: "Target",
                            current_script: to_script.read().clone(),
                            scripts: scripts_data.clone(),
                            on_select: move |id| to_script.set(id),
                        }
                    }
                }

                div { class: "action-footer",
                    button {
                        class: "btn-primary btn-compact",
                        disabled: is_processing() || uploaded_files.read().is_empty() || uploaded_files.read().iter().all(|f| f.status == FileStatus::Done),
                        onclick: convert_files,
                        if is_processing() { "Processing..." } else { "Convert Files" }
                    }
                    button {
                        class: "btn-secondary btn-compact",
                        disabled: is_processing() || uploaded_files.read().is_empty() || uploaded_files.read().iter().any(|f| f.status != FileStatus::Done),
                        onclick: save_all,
                        "Save All"
                    }
                }
            }
        }
    }
}
