class DecorationBase
    extends Component {

    private _skeleton: GameObject;                              // 骨骼
    private skeleton_name_to_transform: Map<string, Transform>; // 骨骼名称到骨骼的映射
    protected player_obj: GameObject;                           // 玩家
    private _dec_grp_tsf: Transform;                            // 服饰组

    @EditorComponentSettings.DecorateName("鼻子")
    public prefab_nose: Prefab;
    @EditorComponentSettings.DecorateName("围脖")
    public prefab_scarf: Prefab;
    @EditorComponentSettings.DecorateName("眼镜")
    public prefab_glasses: Prefab;
    @EditorComponentSettings.DecorateName("眉毛")
    public prefab_eyebrow: Prefab;
    @EditorComponentSettings.DecorateName("帽子")
    public prefab_hat: Prefab;
    @EditorComponentSettings.DecorateName("头饰")
    public prefab_crown: Prefab;
    @EditorComponentSettings.DecorateName("上衣")
    public prefab_top: Prefab;
    @EditorComponentSettings.DecorateName("裤子")
    public prefab_pants: Prefab;
    @EditorComponentSettings.DecorateName("表情")
    public prefab_face: Prefab;
    @EditorComponentSettings.DecorateName("头发")
    public prefab_hair: Prefab;
    @EditorComponentSettings.DecorateName("右脚")
    public prefab_shoes_r: Prefab;
    @EditorComponentSettings.DecorateName("左脚")
    public prefab_shoes_l: Prefab;
    @EditorComponentSettings.DecorateName("翅膀")
    public prefab_wing: Prefab;
    @EditorComponentSettings.DecorateName("尾巴")
    public prefab_tail: Prefab;

    private ori_map = new Map<DecorationType, GameObject>();    // 原始的服饰组
    private dec_map = new Map<DecorationType, Prefab>();        // 穿戴的服饰组

    // 根据服饰类型加载服饰
    public LoadDecoration(type: DecorationType) {

        let ori_go = this.ori_map.get(type);                    // 获取原始服饰
        let asset = this.dec_map.get(type);                     // 获取穿戴的服饰
        if (asset == null) { return; }

        // 如果获取到穿戴的服饰且原始服饰存在，则禁用原始服饰
        if (asset != null && ori_go) {
            ori_go.enable = false;
        }

        // 实例化穿戴的服饰
        let prefab_go = asset.Instance();

        // 加载服饰的静态网格和皮肤
        let tsf_list = new Array<Transform>();
        DecorationStaticMeshLoader.Load(prefab_go, this, tsf_list);
        DecorationSkinLoader.Load(prefab_go, this, tsf_list);

        // 销毁实例化的穿戴的服饰
        GameObject.DestroyGameObject(prefab_go);
    }
    
    public OnStart() {

        // 初始化穿戴的服饰组
        this.dec_map.set(DecorationType.Nose, this.prefab_nose);
        this.dec_map.set(DecorationType.Scarf, this.prefab_scarf);
        this.dec_map.set(DecorationType.Glasses, this.prefab_glasses);
        this.dec_map.set(DecorationType.Eyebrow, this.prefab_eyebrow);
        this.dec_map.set(DecorationType.Hat, this.prefab_hat);
        this.dec_map.set(DecorationType.Crown, this.prefab_crown);
        this.dec_map.set(DecorationType.Hair, this.prefab_hair);
        this.dec_map.set(DecorationType.Top, this.prefab_top);
        this.dec_map.set(DecorationType.Pants, this.prefab_pants);
        this.dec_map.set(DecorationType.Face, this.prefab_face);
        this.dec_map.set(DecorationType.Shoes_r, this.prefab_shoes_r);
        this.dec_map.set(DecorationType.Shoes_l, this.prefab_shoes_l);
        this.dec_map.set(DecorationType.Wing, this.prefab_wing);
        this.dec_map.set(DecorationType.Tail, this.prefab_tail);

        // 初始化原始服饰组
        this.ori_map.set(DecorationType.Hair, this.gameObject.transform.FindChild("hair").gameObject);
        this.ori_map.set(DecorationType.Top, this.gameObject.transform.FindChild("trunk").gameObject);
        this.ori_map.set(DecorationType.Pants, this.gameObject.transform.FindChild("trousers").gameObject);
        this.ori_map.set(DecorationType.Face, this.gameObject.transform.FindChild("face").gameObject);
        this.ori_map.set(DecorationType.Shoes_r, this.gameObject.transform.FindChild("shoes_r").gameObject);
        this.ori_map.set(DecorationType.Shoes_l, this.gameObject.transform.FindChild("shoes_l").gameObject);
        
        // 遍历服饰类别并加载对应服饰
        let cnt = DecorationType.Count;
        for (let i = 0; i < cnt; i++) {
            this.LoadDecoration(i as DecorationType)
        }
    }

    // 获取玩家对象
    public get player(): GameObject {
        if (this.player_obj == null) {
            this.init();
        }
        return this.player_obj;
    }

    // 获取服饰组
    public get decorationGroup(): Transform {
        if (this._dec_grp_tsf == null) {

            if (this.player_obj) {
                let old_scene = SceneManager.activeScene;
                SceneManager.activeScene = this.player_obj.scene;

                let go = new GameObject("DecorationGroup");

                SceneManager.activeScene = old_scene;

                this._dec_grp_tsf = go.transform;
                let player_tsf = this.player_obj.transform;
                this._dec_grp_tsf.SetParent(player_tsf, false);
            }
        }
        return this._dec_grp_tsf;
    }

    // 获取骨骼名称到Transform的映射
    public GetOrGenerateSkeltonNameToTransform(): Map<string, Transform> {
        if (this.skeleton_name_to_transform == null) {
            this.init();
            // 生成骨骼名称到Transform的映射
            this.generate_skel_name_to_transform(this.player_obj.transform);
        }
        return this.skeleton_name_to_transform;
    }

    // 初始化游戏对象及其骨骼
    protected init() {
        if (this.player_obj) { return; }
        this.player_obj = this.gameObject;
        let skeleton_tsf = this.player_obj.transform.FindChild("Bip001");
        if (skeleton_tsf == null) {
            Debug.Error("Decoration: failed to construct decoration with null skeleton. ");
            this.reset();
            return;
        }
        this._skeleton = skeleton_tsf.gameObject;
    }

    private reset() {
        this.player_obj = null;
        this._skeleton = null;
        this.skeleton_name_to_transform = null;
    }

    // 递归骨骼目录
    protected traverse_transform(root: Transform, func: (tsf: Transform) => void) {
        if (root == null) {
            return;
        }
        let cnt = root.childCount;
        func(root);
        for (let i = 0; i < cnt; i++) {
            let child = root.GetChild(i);
            this.traverse_transform(child, func);
        }

    }

    // 将获取骨骼名称存储到Map中
    private generate_skel_name_to_transform(skeleton_tsf: Transform) {
        this.skeleton_name_to_transform = new Map<string, Transform>();

        this.traverse_transform(skeleton_tsf, (tsf: Transform) => {
            if (tsf == null) {
                return;
            }
            this.skeleton_name_to_transform.set(tsf.name, tsf);
        });
    }
}