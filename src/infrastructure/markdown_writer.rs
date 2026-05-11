//! Markdown writer that mirrors a project layout into .md files.
use std::path::Path;
use crate::domain::entities::definition_kind::DefinitionKind;
use crate::domain::entities::documented_construct::DocumentedConstruct;
use crate::domain::entities::parsed_source_file::ParsedSourceFile;
use crate::domain::entities::project_layout::{ ProjectLayout, ProjectEntry };
use crate::domain::ports::index_writer::IndexWriter;
use crate::domain::ports::output_writer::OutputWriter;
use crate::domain::ports::output_writer_error::OutputWriterError;

pub struct MarkdownWriter;

impl MarkdownWriter {
    fn render_markdown(&self, file: &ParsedSourceFile) -> String {
        let file_name = file.source_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        let mut out = format!("# {file_name}\n\n");

        if let Some(doc) = &file.top_level_doc {
            out.push_str(doc);
            out.push_str("\n\n");
        }

        for construct in &file.documented_constructs {
            self.render_construct(&mut out, construct, 2);
        }

        out
    }

    fn render_construct(&self, out: &mut String, construct: &DocumentedConstruct, level: usize) {
        let heading = "#".repeat(level);
        let kind_label = match construct.kind {
            DefinitionKind::Class => "class",
            DefinitionKind::Function => "fn",
            DefinitionKind::Constant => "const",
            DefinitionKind::Module => "module",
        };

        if let Some(sig) = &construct.signature {
            out.push_str(&format!("{heading} `{kind_label} {sig}`\n\n"));
        } else {
            out.push_str(&format!("{heading} `{kind_label} {}`\n\n", construct.name));
        }

        if let Some(doc) = &construct.docstring {
            out.push_str(&format!("> {doc}\n\n"));
        }

        for nested in &construct.nested_constructs {
            self.render_construct(out, nested, level + 1);
        }
    }


    fn write_entries(&self, entries: &[ProjectEntry], project_root_path: &Path, output_root: &Path) -> Result<(), OutputWriterError> {
        for entry in entries {
            match entry {
                ProjectEntry::File { path, parsed_source_file } => {
                    let relative_path = path.strip_prefix(project_root_path).unwrap();
                    let output_path = output_root.join(relative_path).with_extension("md");
                    if let Some(parent) = output_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    let content = self.render_markdown(parsed_source_file);
                    std::fs::write(&output_path, content)?;
                }
                ProjectEntry::Dir { path, child_nodes } => {
                    let relative_path = path.strip_prefix(project_root_path).unwrap();
                    let output_path = output_root.join(relative_path);
                    std::fs::create_dir_all(&output_path)?;
                    self.write_entries(child_nodes, project_root_path, output_root)?;
                }
            }
        }
        Ok(())
    }
}

impl OutputWriter for MarkdownWriter {
    fn write(&self, layout: &ProjectLayout, output_root: &Path) -> Result<(), OutputWriterError> {
        self.write_entries(&layout.entries, &layout.project_root_path, output_root)
    }
}

impl IndexWriter for MarkdownWriter {
    fn write_index(&self, layout: &ProjectLayout, output_root: &Path) -> Result<(), OutputWriterError> {
        let content = self.render_index(layout);
        std::fs::create_dir_all(output_root)?;
        std::fs::write(output_root.join("INDEX.md"), content)?;
        Ok(())
    }
}

impl MarkdownWriter {
    fn render_index(&self, layout: &ProjectLayout) -> String {
        let mut out = String::from("# Index\n\n");
        self.render_index_entries(&mut out, &layout.entries, &layout.project_root_path, 0);
        out
    }

    fn render_index_entries(&self, out: &mut String, entries: &[ProjectEntry], project_root: &Path, depth: usize) {
        let indent = "  ".repeat(depth);
        for entry in entries {
            match entry {
                ProjectEntry::File { path, .. } => {
                    let relative = path.strip_prefix(project_root).unwrap();
                    let md_path = relative.with_extension("md").to_string_lossy().into_owned();
                    let source_path = relative.to_string_lossy().into_owned();
                    out.push_str(&format!("{indent}- [{source_path}]({md_path}) · [source]({})\n", path.display()));
                }
                ProjectEntry::Dir { path, child_nodes } => {
                    let relative = path.strip_prefix(project_root).unwrap();
                    out.push_str(&format!("{indent}- **{}/**\n", relative.to_string_lossy()));
                    self.render_index_entries(out, child_nodes, project_root, depth + 1);
                }
            }
        }
    }
}
