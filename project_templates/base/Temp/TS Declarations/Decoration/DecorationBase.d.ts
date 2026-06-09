declare class DecorationBase extends Component {
    private _skeleton;
    private skeleton_name_to_transform;
    protected player_obj: GameObject;
    private _dec_grp_tsf;
    prefab_nose: Prefab;
    prefab_scarf: Prefab;
    prefab_glasses: Prefab;
    prefab_eyebrow: Prefab;
    prefab_hat: Prefab;
    prefab_crown: Prefab;
    prefab_top: Prefab;
    prefab_pants: Prefab;
    prefab_face: Prefab;
    prefab_hair: Prefab;
    prefab_shoes_r: Prefab;
    prefab_shoes_l: Prefab;
    prefab_wing: Prefab;
    prefab_tail: Prefab;
    private ori_map;
    private dec_map;
    LoadDecoration(type: DecorationType): void;
    OnStart(): void;
    get player(): GameObject;
    get decorationGroup(): Transform;
    GetOrGenerateSkeltonNameToTransform(): Map<string, Transform>;
    protected init(): void;
    private reset;
    protected traverse_transform(root: Transform, func: (tsf: Transform) => void): void;
    private generate_skel_name_to_transform;
}
//# sourceMappingURL=DecorationBase.d.ts.map