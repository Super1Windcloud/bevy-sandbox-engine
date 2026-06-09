class KeyBoardInput extends Component
{
	// ---- 公共属性 ----
	@EditorComponentSettings.DecorateName("是否启用键盘输入")
	public IsKeyBoardInputEnable: boolean = true;	// 是否启用键盘输入

	// ---- 按键设置 ----
	public ArrowKeyForward: KeyCode = KeyCode.W;	// 前进键
	public ArrowKeyBack: KeyCode = KeyCode.S;		// 后退键
	public ArrowKeyLeft: KeyCode = KeyCode.A;		// 左移键
	public ArrowKeyRight: KeyCode = KeyCode.D;		// 右移键
	public FireButton: MouseButton = MouseButton.LeftButton;	// 开火键

	// ---- 引擎 ----
	public OnEnable(): void
	{
		if (!this.IsKeyBoardInputEnable)
		{
			GameObject.DestroyComponent(this);
		}
	}

	public OnUpdate(): void
	{
		this.PlayerMove();
		this.PlayerRotate();
		this.PlayerFire();
	}
	
	private PlayerMove(): void
	{
		let vertical: number = Input.GetKey(this.ArrowKeyForward) ? 1 : (Input.GetKey(this.ArrowKeyBack) ? -1 : 0);
		let horizontal: number = Input.GetKey(this.ArrowKeyRight) ? 1 : (Input.GetKey(this.ArrowKeyLeft) ? -1 : 0);

		let distance: Vector3 = new Vector3(horizontal, 0, vertical);

		// 零向量初始化会导致未定义
		if (!distance.EqualsTo(Vector3.zero))
			distance = distance.normalized;

		GlobalEvent.Instance.Publish(EventName.Player_Controller_Move, distance);
	}

	private PlayerRotate(): void
	{
		let camera = Camera.mainCamera;
		if (camera == null)
			return;

		let mousePoint = Input.mousePosition;
		let mouseRay = camera.ScreenPointToRay(new Vector3(mousePoint.x, mousePoint.y, 0));
		let rayDirection = mouseRay.direction;
		let rotate = new Vector3(rayDirection.x, 0, rayDirection.z);

		if (!rotate.EqualsTo(Vector3.zero))
			rotate = rotate.normalized;

		GlobalEvent.Instance.Publish(EventName.Player_Controller_Rotate, rotate);
	}

	private PlayerFire(): void
	{
		if (Input.GetMouseButton(this.FireButton))
			GlobalEvent.Instance.Publish(EventName.Weapon_Rifle_Shoot, true);
	}
}
