class LevelManager extends Component
{
	@EditorComponentSettings.DecorateName("玩家对象")
	public PlayerObject!: GameObject;
	@EditorComponentSettings.DecorateName("玩家默认位置")
	public PlayerDefaultPos!: GameObject;

	// -- Zombie 实例配置 --
	@EditorComponentSettings.DecorateName("僵尸目标")
	public ZombieTarget!: GameObject;
	@EditorComponentSettings.DecorateName("僵尸预制件")
	public ZombiePrefab!: Prefab;
	@EditorComponentSettings.DecorateName("僵尸实例父级")
	public ZombieInstParent!: Transform;
	@EditorComponentSettings.DecorateName("僵尸实例位置父级")
	public ZombieInstPosParent!: Transform;
	@EditorComponentSettings.DecorateName("僵尸实例间隔")
	public ZombieInstInterval: number = 1;
	@EditorComponentSettings.DecorateName("僵尸实例池化数量")
	public ZombieInstPooledAnount: number = 10;
	@EditorComponentSettings.DecorateName("僵尸实例是否可以增长")
	public ZombieInstwillGrow: boolean = true;

	// -- Zombie 对象池 --
	private _zombieObjPool!: ObjPool;

	// -- 僵尸生成实例计时器 --
	private _currentInstTimer: number = 0;

	// -- 单例 --
	public static Instance: LevelManager | null = null;

	public OnEnable(): void
	{
		if (LevelManager.Instance == null)
			LevelManager.Instance = this;
		else
			GameObject.DestroyGameObject(this.gameObject);
	}

	public OnDestroy(): void
	{
		LevelManager.Instance = null;
	}

	public OnStart(): void
	{
		this._zombieObjPool = new ObjPool();
		this._zombieObjPool.Init(
			this.ZombiePrefab,
			this.ZombieInstPooledAnount,
			this.ZombieInstwillGrow,
			this.ZombieInstParent
		);

		// 游戏开始后初始化玩家状态
		this.PlayerResetPosition();
	}

	public OnUpdate(): void
	{
		if (this.InstTimer())
		{
			let zombie = this._zombieObjPool.GetPooledObject();
			if (zombie != undefined)
			{
				let instPos = this.ZombieInstPosParent.GetChild(this.RandomInt(0, this.ZombieInstPosParent.childCount - 1));
				let zombieComponent = zombie.GetComponent<ZombieSaboteur>(ZombieSaboteur);
				if (zombieComponent == null)
					return;
				zombieComponent.Target = this.ZombieTarget;
				zombieComponent.ResetCharacter();
				zombie.transform.position = instPos.position;
				zombie.enable = true;
			}

			this._currentInstTimer = this.ZombieInstInterval;
		}
	}

	/**
	 * 僵尸生成冷却计时器
	 * @returns 
	 */
	private InstTimer(): boolean
	{
		if (this._currentInstTimer <= 0)
		{
			this._currentInstTimer = 0;
			return true;
		}
		else
		{
			this._currentInstTimer -= Time.deltaTime;
		}

		return false;
	}

	/**
	 * 随机整数
	 * @param min 
	 * @param max 
	 * @returns 
	 */
	private RandomInt(min: number, max: number): number
	{
		min = Math.ceil(min);
		max = Math.floor(max);
		return Math.floor(Math.random() * (max - min + 1)) + min;
	}

	// 更新玩家位置
	public PlayerResetPosition(): void
	{
		this.PlayerObject.transform.position = this.PlayerDefaultPos.transform.position;
		this.PlayerObject.transform.rotation = this.PlayerDefaultPos.transform.rotation;
	}
}
