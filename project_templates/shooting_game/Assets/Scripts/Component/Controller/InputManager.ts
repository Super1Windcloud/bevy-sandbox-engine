class InputManager extends Component
{
	@EditorComponentSettings.Header("输入管理, 请仅启用一套输入")
	@EditorComponentSettings.DecorateName("是否启用键鼠输入")
	public IsEnableKeyBoard: boolean = true;
	@EditorComponentSettings.DecorateName("是否启用摇杆输入")
	public IsEnableJoyStick: boolean = false;

	@EditorComponentSettings.DecorateName("键鼠预制体")
	public KeyBoardPrefab!: Prefab;
	@EditorComponentSettings.DecorateName("摇杆预制体")
	public JoyStickPrefab!: Prefab;

	private _keyBoardGameObject: GameObject | null = null;		// 键鼠对象
	private _joyStickGameObject: GameObject | null = null;		// 摇杆对象

	public OnStart(): void
	{
		if (this.KeyBoardPrefab != null)
		{
			let keyBoard: GameObject = this.KeyBoardPrefab.Instance();
			keyBoard.transform.parent = this.gameObject.transform;
			keyBoard.enable = false;
			this._keyBoardGameObject = keyBoard;
		}

		if (this.JoyStickPrefab != null)
		{
			let JoyStick: GameObject = this.JoyStickPrefab.Instance();
			JoyStick.transform.parent = this.gameObject.transform;
			JoyStick.enable = false;
			this._joyStickGameObject = JoyStick;
		}

		this.ApplyInputMode();
	}

	public OnLateUpdate(): void
	{
		this.ApplyInputMode();
	}

	private ApplyInputMode(): void
	{
		if (this.IsEnableKeyBoard && this.IsEnableJoyStick)
		{
			Debug.Warning("InputManager: keyboard and joystick are both enabled. Joystick input will be disabled.");
			this.IsEnableJoyStick = false;
		}

		if (this._keyBoardGameObject != null)
			this._keyBoardGameObject.enable = this.IsEnableKeyBoard;

		if (this._joyStickGameObject != null)
			this._joyStickGameObject.enable = this.IsEnableJoyStick;
	}
}
