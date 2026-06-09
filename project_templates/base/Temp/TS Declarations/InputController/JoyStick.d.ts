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
//# sourceMappingURL=JoyStick.d.ts.map