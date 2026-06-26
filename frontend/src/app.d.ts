interface IndexResponseItem {
    name: string;
    path: string;
    size: number;
    mime_type: string;
    mtime: number;
    is_dir: boolean;
}

interface IndexResponse {
    path: string;
    base_path?: string;
    items: IndexResponseItem[];
}

interface Window {
    initialConfig: {
        root?: string;
        default_mode: ViewMode;
    };
}

type ViewMode = 'list' | 'gallery';
