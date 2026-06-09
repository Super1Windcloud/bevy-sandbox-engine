class PlayerController extends Component
{
	// ---- 角色移动相关 ----
	@EditorComponentSettings.DecorateName("是否启用视角方向跟随移动方向")
	public IsViewDirFollowMoveDir: boolean = false;
	@EditorComponentSettings.DecorateName("玩家移动速度")
	public MoveSpeed: number = 5;
	@EditorComponentSettings.DecorateName("玩家旋转速度")
	public RotateSpeed: number = 0.3;

	// ---- 私有组件属性 ----
	private _characterController: CharacterController | null = null;	// 角色控制器

	// ---- 私有属性 ----
	private _notControl: boolean = false;				// 是否不可控制
	private _moveDirection: Vector3 = Vector3.zero;		// 移动方向
	private _viewDirection: Vector3 = Vector3.zero;		// 视角方向

	public OnEnable(): void
	{
		GlobalEvent.Instance.Subscribe(EventName.Player_Controller_Move, this.PlayerMoveEvent, this);
		GlobalEvent.Instance.Subscribe(EventName.Player_Controller_Rotate, this.PlayerRotateEvent, this);
	}

	public OnDisable(): void
	{
		GlobalEvent.Instance.UnSubscribe(EventName.Player_Controller_Move, this.PlayerMoveEvent);
		GlobalEvent.Instance.UnSubscribe(EventName.Player_Controller_Rotate, this.PlayerRotateEvent);
	}

	public OnStart(): void
	{
		// 获取角色控制器组件
		this._characterController = this.gameObject.GetComponent<CharacterController>(CharacterController);
		if (this._characterController == null)
		{
			Debug.Warning("WARNING: CharacterController is null!");
		}

		// 将玩家的视角方向设置为当前的正前方
		this._viewDirection = this.transform.forward;
	}

	// ---- 角色移动控制 ----
	/**
	 * 玩家移动
	 * @param moveDirection 移动方向
	 */
	private PlayerMove(moveDirection: Vector3): void
	{
		// 移动角色，如果角色控制器为空则输出警告
		if (this._characterController != null)
			this._characterController.SimpleMove(moveDirection.Mul(this.MoveSpeed));
		else
			Debug.Warning("WARNING: CharacterController is null!");

		// 同步移动方向
		this._moveDirection = moveDirection;

		// 视角跟随移动方向
		// 仍然可以单独旋转角色
		if (this.IsViewDirFollowMoveDir)
			this.PlayerRotate(moveDirection);
	}

	/**
	 * 玩家旋转
	 * @param viewDirection 视角方向
	 */
	private PlayerRotate(viewDirection: Vector3): void
	{
		// 如果视角方向为0则不旋转
		if (viewDirection.magnitude == 0)
			return;

		// 旋转角色
		let rotate = Quaternion.LookRotation(viewDirection);
		this.gameObject.transform.rotation =
			Quaternion.Slerp(
				this.gameObject.transform.rotation,
				rotate,
				this.RotateSpeed
			);

		// 同步旋转
		this._viewDirection = viewDirection;
	}


	// ---- 事件回调 ----
	private PlayerMoveEvent(moveDirection: Vector3): void
	{
		if (this._notControl)
			return;

		this.PlayerMove(moveDirection);
	}

	private PlayerRotateEvent(viewDirection: Vector3): void
	{
		if (this._notControl)
			return;

		this.PlayerRotate(viewDirection);
	}

	// ---- getter & setter ----
	public get MoveDirection(): Vector3 { return this._moveDirection; }
	public get ViewDirection(): Vector3 { return this._viewDirection; }
	public get NotControl(): boolean { return this._notControl; }
	public set NotControl(value: boolean){ this._notControl = value;}
}
