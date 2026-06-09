class ZombieSaboteur extends Component
{
	// ---- 公共属性 ----
	@EditorComponentSettings.DecorateName("移动速度")
	public MoveSpeed: number = 2;
	@EditorComponentSettings.DecorateName("停止距离")
	public StopDistance: number = 2;
	@EditorComponentSettings.DecorateName("伤害")
	public Damage: number = 5;
	@EditorComponentSettings.DecorateName("攻击距离")
	public AttackDistance: number = 1;
	@EditorComponentSettings.DecorateName("攻击间隔")
	public AttackInterval: number = 1;
	@EditorComponentSettings.DecorateName("尸体销毁时间")
	public DestoryBodyTime: number = 2;
	@EditorComponentSettings.DecorateName("目标")
	public Target!: GameObject;
	@EditorComponentSettings.DecorateName("死亡音效")
	public DeadAudio!: AudioSource;
	@EditorComponentSettings.DecorateName("动画控制器")
	public AnimationController!: GameObject;

	// ---- 私有组件属性 ----
	private _zombieAnimation: ZombieAnimation | null = null;				// 动画组件
	private _navMeshAnget: NavMeshAgent | null = null;					// 导航组件
	private _characterController: CharacterController | null = null;		// 角色控制器
	private _healthComponent: CharacterHealth | null = null;				// 生命组件
	private _targetHealthComponent: CharacterHealth | null = null;		// 目标生命组件

	// ---- 私有属性 ----
	private _currentState: ZombieSaboteurStateList = ZombieSaboteurStateList.Idle;		// 当前状态
	private _targetDistance: number = 0;					// 目标距离
	private _attackTimer: number = 0;					// 攻击计时器
	private _deadTimer: number = 0;						// 死亡计时器


	// ---- 引擎 ----
	public OnStart(): void
	{
		// 载入所有组件
		this._characterController = this.gameObject.GetComponent<CharacterController>(CharacterController);
		this._navMeshAnget = this.gameObject.GetComponent<NavMeshAgent>(NavMeshAgent);
		this._healthComponent = this.gameObject.GetComponent<CharacterHealth>(CharacterHealth);
		this._zombieAnimation = this.AnimationController.GetComponent<ZombieAnimation>(ZombieAnimation);
		this._targetHealthComponent = this.Target.GetComponent<CharacterHealth>(CharacterHealth);

		if (this._characterController == null || this._navMeshAnget == null
			|| this._healthComponent == null || this._zombieAnimation == null
			|| this._targetHealthComponent == null)
		{
			Debug.Warning("ZombieSaboteur: required component is null.");
			return;
		}

		// 初始化
		this._navMeshAnget.speed = this.MoveSpeed;
		this._navMeshAnget.stoppingDistance = this.StopDistance;
		this._navMeshAnget.destination = this.Target.transform;
		this._currentState = ZombieSaboteurStateList.Idle;
	}

	public OnUpdate(): void
	{
		if (this._characterController == null || this._navMeshAnget == null
			|| this._healthComponent == null || this._zombieAnimation == null
			|| this._targetHealthComponent == null)
			return;

		// 检查是否死亡
		if (this.CheckHealth())
			this._currentState = ZombieSaboteurStateList.Dead;

		// 驱动角色
		switch (this._currentState)
		{
			case ZombieSaboteurStateList.Idle:
				this.Idle();
				break;
			case ZombieSaboteurStateList.Move:
				this.Move();
				break;
			case ZombieSaboteurStateList.Attack:
				this.Attack();
				break;
			case ZombieSaboteurStateList.Dead:
				this.Dead();
				break;
			default:
				Debug.Warning("ZombieSaboteur: Unknown State.");
				break;
		}

		if (this.Target != null)
		{
			this._targetDistance = this.GetTargetDistance(this.Target.transform.position);
		}
	}


	// ---- 角色状态 ----

	public Idle(): void
	{
		const zombieAnimation = this._zombieAnimation;
		if (zombieAnimation == null)
			return;

		zombieAnimation.PlayAnim("Idle");
		this._currentState = ZombieSaboteurStateList.Move;
	}

	public Move(): void
	{
		const zombieAnimation = this._zombieAnimation;
		const navMeshAgent = this._navMeshAnget;
		if (zombieAnimation == null || navMeshAgent == null)
			return;

		zombieAnimation.PlayAnim("Move");

		// 导航移动
		if (navMeshAgent.enable == false)
			navMeshAgent.enable = true;

		if (this._targetDistance <= this.AttackDistance)
		{
			// 进入攻击范围后转为攻击状态
			navMeshAgent.enable = false;
			zombieAnimation.PlayAnim("Idle");
			this._currentState = ZombieSaboteurStateList.Attack;
		}
	}

	public Attack(): void
	{
		const zombieAnimation = this._zombieAnimation;
		const targetHealthComponent = this._targetHealthComponent;
		if (zombieAnimation == null || targetHealthComponent == null)
			return;

		// 检查是否在攻击范围
		if (this._targetDistance <= this.AttackDistance)
		{
			// 检查是否可以攻击
			if (this._attackTimer >= this.AttackInterval)
			{
				// 攻击目标并归零
				targetHealthComponent.TakeDamage(this.Damage);
				zombieAnimation.PlayAnim("Attack");
				this._attackTimer = 0;
			}
			else
			{
				this._attackTimer += Time.deltaTime;
			}
		}
		else	// 如果不在攻击范围则计时器归零
		{
			this._attackTimer = 0;
			this._currentState = ZombieSaboteurStateList.Move;
		}
	}

	public Dead(): void
	{
		const zombieAnimation = this._zombieAnimation;
		const characterController = this._characterController;
		const navMeshAgent = this._navMeshAnget;
		if (zombieAnimation == null || characterController == null || navMeshAgent == null)
			return;

		zombieAnimation.PlayAnim("Dead");
		if (characterController.enable)
		{
			characterController.enable = false;
			navMeshAgent.enable = false;
		}

		if (this._deadTimer >= this.DestoryBodyTime)
		{
			// 取消激活，返回对象池
			// 还原状态和计时器
			this._deadTimer = 0;
			this._attackTimer = 0;
			this.gameObject.enable = false;
			navMeshAgent.enable = true;
			characterController.enable = true;
		}
		else
		{
			this._deadTimer += Time.deltaTime;
		}
	}


	// ---- 工具 ----

	/**
	 * 获取目标距离
	 * @param targetPos 目标位置
	 * @returns 目标距离
	 */
	protected GetTargetDistance(targetPos: Vector3): number
	{
		let distance: number = Vector3.Distance(
			this.gameObject.transform.position,
			targetPos
		);

		return distance;
	}

	/**
	 * 检查是否死亡
	 * @returns 是否死亡
	 */
	protected CheckHealth(): boolean
	{
		if (this._healthComponent != null && this._healthComponent.IsDead)
			return true;
		return false;
	}

	/**
	 * 设置当前状态
	 * @param state 
	 */
	public SetCurrentState(state: ZombieSaboteurStateList): void
	{
		this._currentState = state;
	}

	public ResetCharacter(): void
	{
		if (this._healthComponent != null && this._characterController != null
			&& this._navMeshAnget != null && this._zombieAnimation != null)
		{
			// 重置生命
			this._healthComponent.RefresHealth();

			// 启用组件
			this._characterController.enable = true;
			this._navMeshAnget.enable = true;

			// 设置初始状态
			this._currentState = ZombieSaboteurStateList.Idle;

			// 播放初始动画
			this._zombieAnimation.PlayAnim("Idle", 0, true);
		}
	}
}

/**
 * 僵尸状态列表
 */
enum ZombieSaboteurStateList
{
	Idle,
	Move,
	Attack,
	Dead,
	DeadAfter
}
