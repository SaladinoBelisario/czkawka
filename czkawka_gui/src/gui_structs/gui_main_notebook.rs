use czkawka_core::common_dir_traversal::CheckingMethod;
use czkawka_core::localizer_core::{fnc_get_similarity_minimal, fnc_get_similarity_very_high};
use gtk4::prelude::*;
use gtk4::{EventControllerKey, GestureClick, TreeView};

use czkawka_core::similar_images::{get_string_from_similarity, Similarity, SIMILAR_VALUES};

use crate::flg;
use crate::help_combo_box::{DUPLICATES_CHECK_METHOD_COMBO_BOX, IMAGES_HASH_SIZE_COMBO_BOX};
use crate::help_functions::get_all_children;
use crate::notebook_enums::{NotebookMainEnum, NUMBER_OF_NOTEBOOK_MAIN_TABS};

#[derive(Clone)]
pub struct GuiMainNotebook {
    pub notebook_main: gtk4::Notebook,

    pub scrolled_window_duplicate_finder: gtk4::ScrolledWindow,
    pub scrolled_window_empty_folder_finder: gtk4::ScrolledWindow,
    pub scrolled_window_empty_files_finder: gtk4::ScrolledWindow,
    pub scrolled_window_temporary_files_finder: gtk4::ScrolledWindow,
    pub scrolled_window_big_files_finder: gtk4::ScrolledWindow,
    pub scrolled_window_similar_images_finder: gtk4::ScrolledWindow,
    pub scrolled_window_similar_videos_finder: gtk4::ScrolledWindow,
    pub scrolled_window_same_music_finder: gtk4::ScrolledWindow,
    pub scrolled_window_invalid_symlinks: gtk4::ScrolledWindow,
    pub scrolled_window_broken_files: gtk4::ScrolledWindow,
    pub scrolled_window_bad_extensions: gtk4::ScrolledWindow,

    pub tree_view_duplicate_finder: gtk4::TreeView,
    pub tree_view_empty_folder_finder: gtk4::TreeView,
    pub tree_view_empty_files_finder: gtk4::TreeView,
    pub tree_view_temporary_files_finder: gtk4::TreeView,
    pub tree_view_big_files_finder: gtk4::TreeView,
    pub tree_view_similar_images_finder: gtk4::TreeView,
    pub tree_view_similar_videos_finder: gtk4::TreeView,
    pub tree_view_same_music_finder: gtk4::TreeView,
    pub tree_view_invalid_symlinks: gtk4::TreeView,
    pub tree_view_broken_files: gtk4::TreeView,
    pub tree_view_bad_extensions: gtk4::TreeView,

    pub evk_tree_view_duplicate_finder: gtk4::EventControllerKey,
    pub evk_tree_view_empty_folder_finder: gtk4::EventControllerKey,
    pub evk_tree_view_empty_files_finder: gtk4::EventControllerKey,
    pub evk_tree_view_temporary_files_finder: gtk4::EventControllerKey,
    pub evk_tree_view_big_files_finder: gtk4::EventControllerKey,
    pub evk_tree_view_similar_images_finder: gtk4::EventControllerKey,
    pub evk_tree_view_similar_videos_finder: gtk4::EventControllerKey,
    pub evk_tree_view_same_music_finder: gtk4::EventControllerKey,
    pub evk_tree_view_invalid_symlinks: gtk4::EventControllerKey,
    pub evk_tree_view_broken_files: gtk4::EventControllerKey,
    pub evk_tree_view_bad_extensions: gtk4::EventControllerKey,

    pub gc_tree_view_duplicate_finder: gtk4::GestureClick,
    pub gc_tree_view_empty_folder_finder: gtk4::GestureClick,
    pub gc_tree_view_empty_files_finder: gtk4::GestureClick,
    pub gc_tree_view_temporary_files_finder: gtk4::GestureClick,
    pub gc_tree_view_big_files_finder: gtk4::GestureClick,
    pub gc_tree_view_similar_images_finder: gtk4::GestureClick,
    pub gc_tree_view_similar_videos_finder: gtk4::GestureClick,
    pub gc_tree_view_same_music_finder: gtk4::GestureClick,
    pub gc_tree_view_invalid_symlinks: gtk4::GestureClick,
    pub gc_tree_view_broken_files: gtk4::GestureClick,
    pub gc_tree_view_bad_extensions: gtk4::GestureClick,

    // General

    // Duplicate
    pub combo_box_duplicate_check_method: gtk4::ComboBoxText,
    pub combo_box_duplicate_hash_type: gtk4::ComboBoxText,
    pub label_duplicate_check_method: gtk4::Label,
    pub label_duplicate_hash_type: gtk4::Label,
    pub check_button_duplicate_case_sensitive_name: gtk4::CheckButton,

    pub image_preview_duplicates: gtk4::Image,

    // Big file
    pub label_big_shown_files: gtk4::Label,
    pub entry_big_files_number: gtk4::Entry,

    // Similar Images
    pub scale_similarity_similar_images: gtk4::Scale,

    pub label_image_resize_algorithm: gtk4::Label,
    pub label_image_hash_type: gtk4::Label,
    pub label_image_hash_size: gtk4::Label,

    pub combo_box_image_resize_algorithm: gtk4::ComboBoxText,
    pub combo_box_image_hash_algorithm: gtk4::ComboBoxText,
    pub combo_box_image_hash_size: gtk4::ComboBoxText,

    pub check_button_image_ignore_same_size: gtk4::CheckButton,
    pub check_button_video_ignore_same_size: gtk4::CheckButton,

    pub check_button_image_fast_compare: gtk4::CheckButton,

    pub label_image_similarity: gtk4::Label,
    pub label_image_similarity_max: gtk4::Label,

    pub image_preview_similar_images: gtk4::Image,
    pub label_similar_images_minimal_similarity: gtk4::Label,

    // Video
    pub label_video_similarity: gtk4::Label,
    pub label_video_similarity_min: gtk4::Label,
    pub label_video_similarity_max: gtk4::Label,

    pub scale_similarity_similar_videos: gtk4::Scale,

    // Music
    pub check_button_music_title: gtk4::CheckButton,
    pub check_button_music_artist: gtk4::CheckButton,
    pub check_button_music_year: gtk4::CheckButton,
    pub check_button_music_bitrate: gtk4::CheckButton,
    pub check_button_music_genre: gtk4::CheckButton,
    pub check_button_music_length: gtk4::CheckButton,
    pub check_button_music_approximate_comparison: gtk4::CheckButton,
}

impl GuiMainNotebook {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let notebook_main: gtk4::Notebook = builder.object("notebook_main").unwrap();

        let scrolled_window_duplicate_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_duplicate_finder").unwrap();
        let scrolled_window_empty_folder_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_empty_folder_finder").unwrap();
        let scrolled_window_empty_files_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_empty_files_finder").unwrap();
        let scrolled_window_temporary_files_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_temporary_files_finder").unwrap();
        let scrolled_window_big_files_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_big_files_finder").unwrap();
        let scrolled_window_similar_images_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_similar_images_finder").unwrap();
        let scrolled_window_similar_videos_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_similar_videos_finder").unwrap();
        let scrolled_window_same_music_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_same_music_finder").unwrap();
        let scrolled_window_invalid_symlinks: gtk4::ScrolledWindow = builder.object("scrolled_window_invalid_symlinks").unwrap();
        let scrolled_window_broken_files: gtk4::ScrolledWindow = builder.object("scrolled_window_broken_files").unwrap();
        let scrolled_window_bad_extensions: gtk4::ScrolledWindow = builder.object("scrolled_window_bad_extensions").unwrap();

        let tree_view_duplicate_finder: gtk4::TreeView = TreeView::new();
        tree_view_duplicate_finder.set_widget_name("PIERD");
        let tree_view_empty_folder_finder: gtk4::TreeView = TreeView::new();
        let tree_view_empty_files_finder: gtk4::TreeView = TreeView::new();
        let tree_view_temporary_files_finder: gtk4::TreeView = TreeView::new();
        let tree_view_big_files_finder: gtk4::TreeView = TreeView::new();
        let tree_view_similar_images_finder: gtk4::TreeView = TreeView::new();
        let tree_view_similar_videos_finder: gtk4::TreeView = TreeView::new();
        let tree_view_same_music_finder: gtk4::TreeView = TreeView::new();
        let tree_view_invalid_symlinks: gtk4::TreeView = TreeView::new();
        let tree_view_broken_files: gtk4::TreeView = TreeView::new();
        let tree_view_bad_extensions: gtk4::TreeView = TreeView::new();

        let evk_tree_view_duplicate_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_duplicate_finder.add_controller(&evk_tree_view_duplicate_finder);
        let evk_tree_view_empty_folder_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_empty_folder_finder.add_controller(&evk_tree_view_empty_folder_finder);
        let evk_tree_view_empty_files_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_empty_files_finder.add_controller(&evk_tree_view_empty_files_finder);
        let evk_tree_view_temporary_files_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_temporary_files_finder.add_controller(&evk_tree_view_temporary_files_finder);
        let evk_tree_view_big_files_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_big_files_finder.add_controller(&evk_tree_view_big_files_finder);
        let evk_tree_view_similar_images_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_similar_images_finder.add_controller(&evk_tree_view_similar_images_finder);
        let evk_tree_view_similar_videos_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_similar_videos_finder.add_controller(&evk_tree_view_similar_videos_finder);
        let evk_tree_view_same_music_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_same_music_finder.add_controller(&evk_tree_view_same_music_finder);
        let evk_tree_view_invalid_symlinks: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_invalid_symlinks.add_controller(&evk_tree_view_invalid_symlinks);
        let evk_tree_view_broken_files: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_broken_files.add_controller(&evk_tree_view_broken_files);
        let evk_tree_view_bad_extensions: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_bad_extensions.add_controller(&evk_tree_view_bad_extensions);

        let gc_tree_view_duplicate_finder: gtk4::GestureClick = GestureClick::new();
        tree_view_duplicate_finder.add_controller(&gc_tree_view_duplicate_finder);
        let gc_tree_view_empty_folder_finder: gtk4::GestureClick = GestureClick::new();
        tree_view_empty_folder_finder.add_controller(&gc_tree_view_empty_folder_finder);
        let gc_tree_view_empty_files_finder: gtk4::GestureClick = GestureClick::new();
        tree_view_empty_files_finder.add_controller(&gc_tree_view_empty_files_finder);
        let gc_tree_view_temporary_files_finder: gtk4::GestureClick = GestureClick::new();
        tree_view_temporary_files_finder.add_controller(&gc_tree_view_temporary_files_finder);
        let gc_tree_view_big_files_finder: gtk4::GestureClick = GestureClick::new();
        tree_view_big_files_finder.add_controller(&gc_tree_view_big_files_finder);
        let gc_tree_view_similar_images_finder: gtk4::GestureClick = GestureClick::new();
        tree_view_similar_images_finder.add_controller(&gc_tree_view_similar_images_finder);
        let gc_tree_view_similar_videos_finder: gtk4::GestureClick = GestureClick::new();
        tree_view_similar_videos_finder.add_controller(&gc_tree_view_similar_videos_finder);
        let gc_tree_view_same_music_finder: gtk4::GestureClick = GestureClick::new();
        tree_view_same_music_finder.add_controller(&gc_tree_view_same_music_finder);
        let gc_tree_view_invalid_symlinks: gtk4::GestureClick = GestureClick::new();
        tree_view_invalid_symlinks.add_controller(&gc_tree_view_invalid_symlinks);
        let gc_tree_view_broken_files: gtk4::GestureClick = GestureClick::new();
        tree_view_broken_files.add_controller(&gc_tree_view_broken_files);
        let gc_tree_view_bad_extensions: gtk4::GestureClick = GestureClick::new();
        tree_view_bad_extensions.add_controller(&gc_tree_view_bad_extensions);

        let combo_box_duplicate_check_method: gtk4::ComboBoxText = builder.object("combo_box_duplicate_check_method").unwrap();
        let combo_box_duplicate_hash_type: gtk4::ComboBoxText = builder.object("combo_box_duplicate_hash_type").unwrap();

        let entry_big_files_number: gtk4::Entry = builder.object("entry_big_files_number").unwrap();

        //// Check Buttons
        let check_button_duplicate_case_sensitive_name: gtk4::CheckButton = builder.object("check_button_duplicate_case_sensitive_name").unwrap();
        let check_button_music_title: gtk4::CheckButton = builder.object("check_button_music_title").unwrap();
        let check_button_music_artist: gtk4::CheckButton = builder.object("check_button_music_artist").unwrap();
        let check_button_music_year: gtk4::CheckButton = builder.object("check_button_music_year").unwrap();
        let check_button_music_bitrate: gtk4::CheckButton = builder.object("check_button_music_bitrate").unwrap();
        let check_button_music_genre: gtk4::CheckButton = builder.object("check_button_music_genre").unwrap();
        let check_button_music_length: gtk4::CheckButton = builder.object("check_button_music_length").unwrap();
        let check_button_music_approximate_comparison: gtk4::CheckButton = builder.object("check_button_music_approximate_comparison").unwrap();

        //// Radio Buttons

        let scale_similarity_similar_images: gtk4::Scale = builder.object("scale_similarity_similar_images").unwrap();
        let scale_similarity_similar_videos: gtk4::Scale = builder.object("scale_similarity_similar_videos").unwrap();

        let check_button_image_fast_compare: gtk4::CheckButton = builder.object("check_button_image_fast_compare").unwrap();

        let combo_box_image_resize_algorithm: gtk4::ComboBoxText = builder.object("combo_box_image_resize_algorithm").unwrap();
        let combo_box_image_hash_algorithm: gtk4::ComboBoxText = builder.object("combo_box_image_hash_algorithm").unwrap();
        let combo_box_image_hash_size: gtk4::ComboBoxText = builder.object("combo_box_image_hash_size").unwrap();

        let check_button_image_ignore_same_size: gtk4::CheckButton = builder.object("check_button_image_ignore_same_size").unwrap();
        let check_button_video_ignore_same_size: gtk4::CheckButton = builder.object("check_button_video_ignore_same_size").unwrap();

        let label_similar_images_minimal_similarity: gtk4::Label = builder.object("label_similar_images_minimal_similarity").unwrap();

        let label_duplicate_check_method: gtk4::Label = builder.object("label_duplicate_check_method").unwrap();
        let label_duplicate_hash_type: gtk4::Label = builder.object("label_duplicate_hash_type").unwrap();
        let label_big_shown_files: gtk4::Label = builder.object("label_big_shown_files").unwrap();
        let label_image_resize_algorithm: gtk4::Label = builder.object("label_image_resize_algorithm").unwrap();
        let label_image_hash_type: gtk4::Label = builder.object("label_image_hash_type").unwrap();
        let label_image_hash_size: gtk4::Label = builder.object("label_image_hash_size").unwrap();
        let label_image_similarity: gtk4::Label = builder.object("label_image_similarity").unwrap();
        let label_image_similarity_max: gtk4::Label = builder.object("label_image_similarity_max").unwrap();
        let label_video_similarity: gtk4::Label = builder.object("label_video_similarity").unwrap();
        let label_video_similarity_min: gtk4::Label = builder.object("label_video_similarity_min").unwrap();
        let label_video_similarity_max: gtk4::Label = builder.object("label_video_similarity_max").unwrap();

        let image_preview_similar_images: gtk4::Image = builder.object("image_preview_similar_images").unwrap();
        let image_preview_duplicates: gtk4::Image = builder.object("image_preview_duplicates").unwrap();

        Self {
            notebook_main,
            scrolled_window_duplicate_finder,
            scrolled_window_empty_folder_finder,
            scrolled_window_empty_files_finder,
            scrolled_window_temporary_files_finder,
            scrolled_window_big_files_finder,
            scrolled_window_similar_images_finder,
            scrolled_window_similar_videos_finder,
            scrolled_window_same_music_finder,
            scrolled_window_invalid_symlinks,
            scrolled_window_broken_files,
            scrolled_window_bad_extensions,
            tree_view_duplicate_finder,
            tree_view_empty_folder_finder,
            tree_view_empty_files_finder,
            tree_view_temporary_files_finder,
            tree_view_big_files_finder,
            tree_view_similar_images_finder,
            tree_view_similar_videos_finder,
            tree_view_same_music_finder,
            tree_view_invalid_symlinks,
            tree_view_broken_files,
            tree_view_bad_extensions,
            evk_tree_view_duplicate_finder,
            evk_tree_view_empty_folder_finder,
            evk_tree_view_empty_files_finder,
            evk_tree_view_temporary_files_finder,
            evk_tree_view_big_files_finder,
            evk_tree_view_similar_images_finder,
            evk_tree_view_similar_videos_finder,
            evk_tree_view_same_music_finder,
            evk_tree_view_invalid_symlinks,
            evk_tree_view_broken_files,
            entry_big_files_number,
            check_button_music_title,
            check_button_music_artist,
            check_button_music_year,
            check_button_music_bitrate,
            check_button_music_genre,
            check_button_music_length,
            check_button_music_approximate_comparison,
            scale_similarity_similar_images,
            scale_similarity_similar_videos,
            check_button_image_ignore_same_size,
            label_similar_images_minimal_similarity,
            label_duplicate_check_method,
            label_duplicate_hash_type,
            combo_box_duplicate_check_method,
            label_big_shown_files,
            label_image_resize_algorithm,
            label_image_hash_type,
            label_image_hash_size,
            combo_box_image_resize_algorithm,
            combo_box_image_hash_algorithm,
            label_image_similarity,
            label_image_similarity_max,
            label_video_similarity,
            label_video_similarity_min,
            label_video_similarity_max,
            image_preview_similar_images,
            image_preview_duplicates,
            combo_box_duplicate_hash_type,
            combo_box_image_hash_size,
            check_button_video_ignore_same_size,
            check_button_image_fast_compare,
            check_button_duplicate_case_sensitive_name,
            evk_tree_view_bad_extensions,
            gc_tree_view_duplicate_finder,
            gc_tree_view_empty_folder_finder,
            gc_tree_view_empty_files_finder,
            gc_tree_view_temporary_files_finder,
            gc_tree_view_big_files_finder,
            gc_tree_view_similar_images_finder,
            gc_tree_view_similar_videos_finder,
            gc_tree_view_same_music_finder,
            gc_tree_view_invalid_symlinks,
            gc_tree_view_broken_files,
            gc_tree_view_bad_extensions,
        }
    }

    pub fn get_main_tree_views(&self) -> [TreeView; NUMBER_OF_NOTEBOOK_MAIN_TABS] {
        [
            self.tree_view_duplicate_finder.clone(),
            self.tree_view_empty_folder_finder.clone(),
            self.tree_view_big_files_finder.clone(),
            self.tree_view_empty_files_finder.clone(),
            self.tree_view_temporary_files_finder.clone(),
            self.tree_view_similar_images_finder.clone(),
            self.tree_view_similar_videos_finder.clone(),
            self.tree_view_same_music_finder.clone(),
            self.tree_view_invalid_symlinks.clone(),
            self.tree_view_broken_files.clone(),
            self.tree_view_bad_extensions.clone(),
        ]
    }

    pub fn update_language(&self) {
        self.check_button_duplicate_case_sensitive_name.set_label(Some(&flg!("duplicate_case_sensitive_name")));
        self.check_button_music_title.set_label(Some(&flg!("music_title_checkbox")));
        self.check_button_music_artist.set_label(Some(&flg!("music_artist_checkbox")));
        self.check_button_music_year.set_label(Some(&flg!("music_year_checkbox")));
        self.check_button_music_bitrate.set_label(Some(&flg!("music_bitrate_checkbox")));
        self.check_button_music_genre.set_label(Some(&flg!("music_genre_checkbox")));
        self.check_button_music_length.set_label(Some(&flg!("music_length_checkbox")));
        self.check_button_music_approximate_comparison.set_label(Some(&flg!("music_comparison_checkbox")));

        self.check_button_music_approximate_comparison
            .set_tooltip_text(Some(&flg!("music_comparison_checkbox_tooltip")));

        self.label_duplicate_check_method.set_label(&flg!("main_label_check_method"));
        self.label_duplicate_hash_type.set_label(&flg!("main_label_hash_type"));
        self.label_big_shown_files.set_label(&flg!("main_label_shown_files"));
        self.label_image_resize_algorithm.set_label(&flg!("main_label_resize_algorithm"));
        self.label_image_hash_type.set_label(&flg!("main_label_hash_type"));
        self.label_image_hash_size.set_label(&flg!("main_label_hash_size"));
        self.label_image_similarity.set_label(&flg!("main_label_similarity"));
        self.label_image_similarity_max.set_label(&fnc_get_similarity_very_high());
        self.label_video_similarity.set_label(&flg!("main_label_similarity"));
        self.label_video_similarity_min.set_label(&fnc_get_similarity_minimal());
        self.label_video_similarity_max.set_label(&fnc_get_similarity_very_high());

        self.label_duplicate_check_method.set_tooltip_text(Some(&flg!("duplicate_check_method_tooltip")));
        self.combo_box_duplicate_check_method.set_tooltip_text(Some(&flg!("duplicate_check_method_tooltip")));
        self.label_duplicate_hash_type.set_tooltip_text(Some(&flg!("duplicate_hash_type_tooltip")));
        self.combo_box_duplicate_hash_type.set_tooltip_text(Some(&flg!("duplicate_hash_type_tooltip")));
        self.check_button_duplicate_case_sensitive_name
            .set_tooltip_text(Some(&flg!("duplicate_case_sensitive_name_tooltip")));

        self.combo_box_image_hash_size.set_tooltip_text(Some(&flg!("image_hash_size_tooltip")));
        self.label_image_hash_size.set_tooltip_text(Some(&flg!("image_hash_size_tooltip")));

        self.combo_box_image_resize_algorithm.set_tooltip_text(Some(&flg!("image_resize_filter_tooltip")));
        self.label_image_resize_algorithm.set_tooltip_text(Some(&flg!("image_resize_filter_tooltip")));

        self.combo_box_image_hash_algorithm.set_tooltip_text(Some(&flg!("image_hash_alg_tooltip")));
        self.label_image_hash_type.set_tooltip_text(Some(&flg!("image_hash_alg_tooltip")));

        self.check_button_image_ignore_same_size
            .set_tooltip_text(Some(&flg!("check_button_general_same_size_tooltip")));
        self.check_button_video_ignore_same_size
            .set_tooltip_text(Some(&flg!("check_button_general_same_size_tooltip")));
        self.check_button_image_ignore_same_size.set_label(Some(&flg!("check_button_general_same_size")));
        self.check_button_video_ignore_same_size.set_label(Some(&flg!("check_button_general_same_size")));

        self.check_button_image_fast_compare.set_label(Some(&flg!("main_notebook_image_fast_compare")));
        self.check_button_image_fast_compare
            .set_tooltip_text(Some(&flg!("main_notebook_image_fast_compare_tooltip")));

        {
            let hash_size_index = self.combo_box_image_hash_size.active().unwrap() as usize;
            let hash_size = IMAGES_HASH_SIZE_COMBO_BOX[hash_size_index];
            match hash_size {
                8 => {
                    self.label_similar_images_minimal_similarity
                        .set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[0][5]), 8));
                }
                16 => {
                    self.label_similar_images_minimal_similarity
                        .set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[1][5]), 16));
                }
                32 => {
                    self.label_similar_images_minimal_similarity
                        .set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[2][5]), 32));
                }
                64 => {
                    self.label_similar_images_minimal_similarity
                        .set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[3][5]), 64));
                }
                _ => panic!(),
            }
        }

        let vec_children: Vec<gtk4::Widget> = get_all_children(&self.notebook_main);
        let vec_children: Vec<gtk4::Widget> = get_all_children(&vec_children[1]);

        // Change name of main notebook tabs
        for (main_enum, fl_thing) in [
            (NotebookMainEnum::Duplicate as usize, flg!("main_notebook_duplicates")),
            (NotebookMainEnum::EmptyDirectories as usize, flg!("main_notebook_empty_directories")),
            (NotebookMainEnum::BigFiles as usize, flg!("main_notebook_big_files")),
            (NotebookMainEnum::EmptyFiles as usize, flg!("main_notebook_empty_files")),
            (NotebookMainEnum::Temporary as usize, flg!("main_notebook_temporary")),
            (NotebookMainEnum::SimilarImages as usize, flg!("main_notebook_similar_images")),
            (NotebookMainEnum::SimilarVideos as usize, flg!("main_notebook_similar_videos")),
            (NotebookMainEnum::SameMusic as usize, flg!("main_notebook_same_music")),
            (NotebookMainEnum::Symlinks as usize, flg!("main_notebook_symlinks")),
            (NotebookMainEnum::BrokenFiles as usize, flg!("main_notebook_broken_files")),
            (NotebookMainEnum::BadExtensions as usize, flg!("main_notebook_bad_extensions")),
        ] {
            self.notebook_main
                .tab_label(&vec_children[main_enum])
                .unwrap()
                .downcast::<gtk4::Label>()
                .unwrap()
                .set_text(&fl_thing);
        }

        // Change names of columns
        let names_of_columns = [
            vec![
                flg!("main_tree_view_column_size"),
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_modification"),
            ], // Duplicates
            vec![
                flg!("main_tree_view_column_folder_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_modification"),
            ], // Empty Folders
            vec![
                flg!("main_tree_view_column_size"),
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_modification"),
            ], // Big files
            vec![
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_modification"),
            ], // Empty files
            vec![
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_modification"),
            ], // Temporary Files
            vec![
                flg!("main_tree_view_column_similarity"),
                flg!("main_tree_view_column_size"),
                flg!("main_tree_view_column_dimensions"),
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_modification"),
            ], // Similar Images
            vec![
                flg!("main_tree_view_column_size"),
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_modification"),
            ], // Similar Videos
            vec![
                flg!("main_tree_view_column_size"),
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_title"),
                flg!("main_tree_view_column_artist"),
                flg!("main_tree_view_column_year"),
                flg!("main_tree_view_column_bitrate"),
                flg!("main_tree_view_column_length"),
                flg!("main_tree_view_column_genre"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_modification"),
            ], // Music Dupliactes
            vec![
                flg!("main_tree_view_column_symlink_file_name"),
                flg!("main_tree_view_column_symlink_folder"),
                flg!("main_tree_view_column_destination_path"),
                flg!("main_tree_view_column_type_of_error"),
                flg!("main_tree_view_column_modification"),
            ], // Invalid Symlinks
            vec![
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_type_of_error"),
                flg!("main_tree_view_column_modification"),
            ], // Broken Files
            vec![
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_current_extension"),
                flg!("main_tree_view_column_proper_extensions"),
                flg!("main_tree_view_column_modification"),
            ], // Broken Files
        ];

        for (notebook_index, tree_view) in self.get_main_tree_views().iter().enumerate() {
            for (column_index, column) in tree_view.columns().iter().enumerate() {
                if column_index == 0 {
                    continue; // Selection button
                }
                column.set_title(&names_of_columns[notebook_index][column_index - 1]);
            }
        }

        {
            let active = self.combo_box_duplicate_check_method.active().unwrap_or(0);
            self.combo_box_duplicate_check_method.remove_all();
            for i in &DUPLICATES_CHECK_METHOD_COMBO_BOX {
                let text = match i.check_method {
                    CheckingMethod::Hash => flg!("duplicate_mode_hash_combo_box"),
                    CheckingMethod::Size => flg!("duplicate_mode_size_combo_box"),
                    CheckingMethod::Name => flg!("duplicate_mode_name_combo_box"),
                    _ => {
                        panic!()
                    }
                };
                self.combo_box_duplicate_check_method.append_text(&text);
            }
            self.combo_box_duplicate_check_method.set_active(Some(active));
        }
    }
}
