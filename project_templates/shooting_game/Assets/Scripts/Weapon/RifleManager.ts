class RifleManager extends Component
{
	// ---- 步枪和子弹预制体 ----
	@EditorComponentSettings.DecorateName("步枪预制件")
	public RiflePrefab!: Prefab;
	@EditorComponentSettings.DecorateName("步枪子弹预制件")
	public RifleBulletPrefab!: Prefab;
	@EditorComponentSettings.DecorateName("步枪子弹实例化父物体")
	public RifleBulletInstParent!: Transform;

	// ---- 步枪配置 ----
	@EditorComponentSettings.DecorateName("弹夹容量")
	public MagazineCapacity: number = 30;
	@EditorComponentSettings.DecorateName("射击间隔")
	public ShootingInterval: number = 0.1;
	@EditorComponentSettings.DecorateName("换弹时间")
	public RifleReloadTime: number = 4;
	@EditorComponentSettings.DecorateName("枪械实例化位置")
	public RifleInstTransform!: Transform;
	@EditorComponentSettings.DecorateName("枪械位置偏移")
	public RifleTransformOffset!: Vector3;
	@EditorComponentSettings.DecorateName("枪械旋转偏移")
	public RifleRotationOffset!: Vector3;
	@EditorComponentSettings.DecorateName("枪械开火点名字")
	public RifleFirePointName!: string;

	// ---- 角色 IK ----
	@EditorComponentSettings.DecorateName("是否启用枪械IK")
	public IsEnableRifleIK!: boolean;
	@EditorComponentSettings.DecorateName("角色左手IK对象")
	public CharacterLeftHand!: GameObject;
	@EditorComponentSettings.DecorateName("枪械左手IK点名称")
	public RifleLeftPointName!: string;

	// ---- 步枪生成时配置属性 ----
	private _rifleInst!: GameObject;				// 枪械实例
	private _rifleFirePointName: Transform | null = null;		// 枪械开火点
	private _bulletObjPool!: ObjPool;			// 子弹对象池
	private _rifleLeftPoint: Transform | null = null;			// 枪械左手IK点
	private _rifleMuzzleFlash: ParticleSystem | null = null;	// 枪械开火特效
	private _rifleMuzzleAudio: AudioSource | null = null;		// 枪械开火音效

	// ---- 步枪当前属性 ----
	private _currentMagazine: number = 0;		// 当前弹夹容量
	private _currentReloadTime: number = 0;		// 当前换弹时间
	private _currentFireCooldown: number = 0;	// 当前冷却时间
	private _isReloading: boolean = false;		// 是否正在换弹
	private _isHolderDead: boolean = false;		// 是否持枪者死亡


	public OnEnable(): void
	{
		// 订阅射击事件
		GlobalEvent.Instance.Subscribe(EventName.Weapon_Rifle_Shoot, this.WeaponAttack, this);
		// 订阅持枪者状态变化事件
		GlobalEvent.Instance.Subscribe(EventName.Weapon_Rifle_HolderState, this.SetHolderDead, this);
	}

	public OnDestroy(): void
	{
		GlobalEvent.Instance.UnSubscribe(EventName.Weapon_Rifle_Shoot, this.WeaponAttack);
		GlobalEvent.Instance.UnSubscribe(EventName.Weapon_Rifle_HolderState, this.SetHolderDead);
	}

	public OnStart(): void
	{
		// 创建子弹对象池
		this._bulletObjPool = new ObjPool();
		this._bulletObjPool.Init(this.RifleBulletPrefab, 20, true, this.RifleBulletInstParent);

		// 实例化枪械
		let rifle: GameObject = this.RiflePrefab.Instance();
		rifle.transform.parent = this.RifleInstTransform;
		if (this.RifleTransformOffset != Vector3.zero || this.RifleRotationOffset != Vector3.zero)
		{
			rifle.transform.localPosition = this.RifleTransformOffset;
			rifle.transform.localRotation = Quaternion.FromEuler(this.RifleRotationOffset);
		}
		this._rifleInst = rifle;

		// 获取枪械属性
		let rifleAttribute: RifleAttributes | null = this._rifleInst.GetComponent<RifleAttributes>(RifleAttributes);
		if (rifleAttribute != null)
		{
			this._rifleMuzzleFlash = rifleAttribute.GetRifleShootEffect;
			this._rifleMuzzleAudio = rifleAttribute.GetRifleShootAudio;
		}

		// 初始化属性
		this._currentMagazine = this.MagazineCapacity;
		this._currentReloadTime = this.RifleReloadTime;
		this._rifleLeftPoint = this._rifleInst.transform.FindChild(this.RifleLeftPointName);
		this._rifleFirePointName = this._rifleInst.transform.FindChild(this.RifleFirePointName);

		// 枪械IK
		if (this.IsEnableRifleIK)
		{
			let ik: FABRIK | null = this.CharacterLeftHand.GetComponent<FABRIK>(FABRIK);

			if (ik != null && this._rifleLeftPoint != null)
				ik.target = this._rifleLeftPoint;
		}
	}

	/**
	 * 枪械射击
	 */
	public WeaponAttack(): void
	{
		// 检查持枪者是否死亡
		if (this._isHolderDead)
			return;

		// 检查当前冷却时间
		if (this._currentFireCooldown > 0)
			return;

		// 当前是否在换弹
		if (this._isReloading)
			return;

		// 获取子弹并发射
		let bullet: GameObject | undefined = this._bulletObjPool.GetPooledObject();
		if (bullet == undefined || this._rifleFirePointName == null)
			return;

		bullet.transform.position = this._rifleFirePointName.position;
		bullet.transform.rotation = this._rifleFirePointName.rotation;
		if (this._rifleMuzzleFlash != null)
			this._rifleMuzzleFlash.Play();
		if (this._rifleMuzzleAudio != null)
			this._rifleMuzzleAudio.Play();
		
		bullet.enable = true;

		// 弹夹子弹更新
		this._currentMagazine -= 1;

		// 重置枪械冷却
		this._currentFireCooldown = this.ShootingInterval;
	}

	/**
	 * 枪械冷却
	 */
	private Cooldown(): void
	{
		if (this._currentFireCooldown > 0)
			this._currentFireCooldown -= Time.deltaTime;
	}

	/**
	 * 枪械换弹
	 */
	private Reload(): void
	{
		if (this._currentMagazine <= 0)
		{
			if (this._isReloading == false)
				this._isReloading = true;

			this._currentReloadTime -= Time.deltaTime;
		}

		if (this._currentReloadTime <= 0)
		{
			this._currentMagazine = this.MagazineCapacity;
			this._isReloading = false;
			this._currentReloadTime = this.RifleReloadTime;
		}
	}

	public OnUpdate(): void
	{
		this.Reload();
		this.Cooldown();
	}

	/**
	 * 更新枪械状态
	 * @param state 
	 */
	private SetHolderDead(state: boolean): void
	{
		this._isHolderDead = state;
	}

	// ---- getter & setter 访问器 ----
	public get CurrentMagazine(): number { return this._currentMagazine; }
	public get CurrentReloadTime(): number { return this._currentReloadTime; }
	public get CurrentFireCooldown(): number { return this._currentFireCooldown; }
	public get IsReloading(): boolean { return this._isReloading; }
	public get IsHolderDead(): boolean { return this._isHolderDead; }

	public set IsHolderDead(state: boolean) { this._isHolderDead = state; }
}
