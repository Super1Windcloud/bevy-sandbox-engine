class PlayerManager extends Component
{
	@EditorComponentSettings.DecorateName("是否启用重置玩家")
	public EnableResetPlayer: boolean = false;
	@EditorComponentSettings.DecorateName("重置玩家计时器")
	public ResetPlayerTimer: number = 5;

	private _currentResetPlayerTimer: number = 0;	// 当前重置玩家计时器

	private _playerHealth: CharacterHealth | null = null;			// 玩家生命值组件
	private _playerController: PlayerController | null = null;	// 玩家控制器组件

	public OnStart(): void
	{
		// 获取组件
		this._playerHealth = this.gameObject.GetComponent<CharacterHealth>(CharacterHealth);
		this._playerController = this.gameObject.GetComponent<PlayerController>(PlayerController);

		// 组件检查
		if (this._playerHealth == null)
			Debug.Warning("PlayerManager: PlayerHealth is null.");
		if (this._playerController == null)
			Debug.Warning("PlayerManager: PlayerController is null.");
	}

	public OnUpdate(): void
	{
		if (this._playerHealth == null || this._playerController == null)
			return;

		if (this._playerHealth.IsDead)
		{
			// 发布玩家死亡事件
			GlobalEvent.Instance.Publish(EventName.Weapon_Rifle_HolderState, true);

			// 玩家死亡后，不可控制
			if (this._playerController.NotControl == false)
				this._playerController.NotControl = true;


			// 若不启用自动复活，则不进行后续操作
			if (!this.EnableResetPlayer)
				return;

			// 计算复活计时器
			if (this._currentResetPlayerTimer >= this.ResetPlayerTimer)
			{
				// 复活玩家
				this._playerHealth.RefresHealth();
				this._playerController.NotControl = false;
				this._currentResetPlayerTimer = 0;
				GlobalEvent.Instance.Publish(EventName.Weapon_Rifle_HolderState, false);
			}
			else
			{
				this._currentResetPlayerTimer += Time.deltaTime;
			}
		}
	}

	// ---- getter & setter ----
	public get IsDead(): boolean { return this._playerHealth?.IsDead ?? false; }
	public get NotControl(): boolean { return this._playerController?.NotControl ?? true; }
	public get MoveDirection(): Vector3 { return this._playerController?.MoveDirection ?? Vector3.zero; }
	public get ViewDirection(): Vector3 { return this._playerController?.ViewDirection ?? Vector3.zero; }
	public set NotControl(value: boolean)
	{
		if (this._playerController != null)
			this._playerController.NotControl = value;
	}
}
