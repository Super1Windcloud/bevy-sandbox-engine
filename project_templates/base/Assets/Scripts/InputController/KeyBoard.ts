class KeyBoard extends Component
{
	@EditorComponentSettings.DecorateName("是否启用键盘输入")
	public IsKeyBoardInputEnable: boolean = true;

	// ---- 控制实体 ----
	@EditorComponentSettings.DecorateName("控制目标")
	public ControlTarget: PlayerController;

	// ---- 按键设置 ----
	public ArrowKeyForward: KeyCode = KeyCode.W;	// 前进键
	public ArrowKeyBack: KeyCode = KeyCode.S;		// 后退键
	public ArrowKeyLeft: KeyCode = KeyCode.A;		// 左移键
	public ArrowKeyRight: KeyCode = KeyCode.D;		// 右移键

	// ---- 引擎 ----
	public OnEnable(): void
	{
		if (!this.IsKeyBoardInputEnable)
		{
			this.enable = false;
		}
	}

	public OnUpdate(): void
	{
		this.PlayerMove();
		this.PlayerJump();
	}

	private PlayerMove(): void
	{
		let vertical: number = Input.GetKey(KeyCode.W) ? 1 : (Input.GetKey(KeyCode.S) ? -1 : 0);
		let horizontal: number = Input.GetKey(KeyCode.D) ? 1 : (Input.GetKey(KeyCode.A) ? -1 : 0);

		let distance: Vector2 = new Vector2(horizontal, vertical);

		// 零向量归一化会导致未定义
		if (!distance.EqualsTo(Vector2.zero))
			distance = distance.normalized;

		this.ControlTarget.CharacterMove(distance);
	}

	private PlayerJump(): void
	{
		if (Input.GetKeyDown(KeyCode.Space))
			this.ControlTarget.CharacterJump();
	}
}