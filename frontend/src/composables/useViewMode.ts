import { inject } from "vue";

const useViewMode = () => {
    return inject<ViewMode>('viewMode')!;
}

export default useViewMode;
