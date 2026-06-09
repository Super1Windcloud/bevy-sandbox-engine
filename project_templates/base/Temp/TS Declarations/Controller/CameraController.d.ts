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
//# sourceMappingURL=CameraController.d.ts.map