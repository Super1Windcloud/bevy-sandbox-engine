declare class CameraController extends Component {
    CameraFollow: GameObject;
    MaxDistance: number;
    MinDistance: number;
    MaxRotateAngle: number;
    MinRotateAngle: number;
    RotateSensitivity: number;
    HitLayerMask: number;
    Offset: number;
    SmoothSpeed: number;
    private _viewHorizontal;
    private _viewVertical;
    private _yaw;
    private _pitch;
    private _outDistance;
    OnLateUpdate(): void;
    private CameraMove;
    CameraView(vec: Vector2): void;
}
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
declare class PlayerController extends Component {
    Gravity: number;
    Animator: Animator;
    Cam: Camera;
    private _characterController;
    private _camController;
    MoveSpeed: number;
    JumpSpeed: number;
    JumpHeight: number;
    GroundedOffset: number;
    RotateSpeed: number;
    RotationSmoothTime: number;
    private _moveDirection;
    private _moveHorizontal;
    private _moveVertical;
    private _groundDirection;
    private _isJumping;
    private _isGrounded;
    CheckRadius: number;
    CheckDistance: number;
    CheckMask: number;
    ShowGroundEetection: boolean;
    OnStart(): void;
    OnUpdate(): void;
    private GroundedCheck;
    private JumpAndGravity;
    private Move;
    CharacterMove(vec: Vector2): void;
    CharacterView(vec: Vector2): void;
    CharacterJump(): void;
}
declare class DecorationSkinLoader {
    static Load(prefab_obj: GameObject, dec_ctrl: DecorationBase, list: Array<Transform>): Array<Transform>;
}
declare class DecorationStaticMeshLoader {
    static Load(prefab_obj: GameObject, dec_ctrl: DecorationBase, list: Array<Transform>): any;
}
declare class JoyStick extends Component {
    IsJoyStickInputEnable: boolean;
    ControlTarget: PlayerController;
    LeftJoyStick: string;
    LeftJoyStickCap: string;
    ScreenMoveArea: string;
    ScreenMoveDeadZone: number;
    JumpButton: string;
    private _uiComponent;
    private _leftJoyStick;
    private _leftJoyStickCap;
    private _jumpButton;
    private _screenMoveArea;
    OnStart(): void;
    private PlayerMove;
}
declare class KeyBoard extends Component {
    IsKeyBoardInputEnable: boolean;
    ControlTarget: PlayerController;
    ArrowKeyForward: KeyCode;
    ArrowKeyBack: KeyCode;
    ArrowKeyLeft: KeyCode;
    ArrowKeyRight: KeyCode;
    OnEnable(): void;
    OnUpdate(): void;
    private PlayerMove;
    private PlayerJump;
}
declare class MathfExpand {
    static rotateVectorByQuaternion(q: Quaternion, v: Vector3): Vector3;
}
declare enum DecorationType {
    Nose = 0,
    Scarf = 1,
    Glasses = 2,
    Eyebrow = 3,
    Hat = 4,
    Crown = 5,
    Hair = 6,
    Face = 7,
    Top = 8,
    Pants = 9,
    Shoes_r = 10,
    Shoes_l = 11,
    Wing = 12,
    Tail = 13,
    Count = 14
}
//# sourceMappingURL=Assembly.d.ts.map