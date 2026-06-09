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
//# sourceMappingURL=PlayerController.d.ts.map