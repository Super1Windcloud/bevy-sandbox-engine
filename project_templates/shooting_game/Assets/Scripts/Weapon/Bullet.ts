class RifleBullet extends Component
{
	// -- 子弹配置属性 --
	@EditorComponentSettings.DecorateName("子弹飞行速度")
	public BulletSpeed!: number;
	@EditorComponentSettings.DecorateName("子弹伤害")
	public BulletDamage!: number;
	@EditorComponentSettings.DecorateName("子弹检测敌人层级")
	public BulletEnemyLayer!: number;
	@EditorComponentSettings.DecorateName("子弹射程")
	public BulletRange!: number;

	// -- 私有属性 --
	@EditorComponentSettings.DecorateName("子弹刚体组件")
	private _bulletRigidbody: Rigidbody | null = null;
	@EditorComponentSettings.DecorateName("子弹粒子系统组件")
	private _particleSystem: ParticleSystem | null = null;
	@EditorComponentSettings.DecorateName("子弹飞行计时器")
	private _bulletFlyTimer: number = 0;
	@EditorComponentSettings.DecorateName("当前子弹飞行时间")
	private _currentFlyTimer: number = 0;

	public OnEnable(): void
	{
		this._bulletRigidbody = this.gameObject.GetComponent<Rigidbody>(Rigidbody);

		if (this._bulletRigidbody != null)
			this.Attack();

		this.InitialCollisionCheck();
	}

	public OnStart(): void
	{
		this._bulletFlyTimer = this.BulletRange / this.BulletSpeed;

		this._bulletRigidbody = this.gameObject.GetComponent<Rigidbody>(Rigidbody);
		if (this._bulletRigidbody != null)
			this.Attack();
	}
	
	public OnUpdate(): void
	{
		if (this.BulletFlyTimer())
		{
			this.gameObject.enable = false;
		}
	}

	public OnFixedUpdate(): void
	{
		this.InitialCollisionCheck();
	}

	public OnDisable(): void
	{
		if (this._bulletRigidbody != null)
		{
			this._bulletRigidbody.velocity = Vector3.zero;
			this._bulletRigidbody.angularVelocity = Vector3.zero;
		}
		this.gameObject.transform.localPosition = Vector3.zero;

		if (this._particleSystem != null)
		{
			this._particleSystem.Stop(PSStopBehavior.StopEmittingAndClear);
		}
	}

	private Attack(): void
	{
		if (this._bulletRigidbody == null)
			return;

		this._bulletRigidbody.velocity = this.transform.up.Mul(this.BulletSpeed);
	}

	private InitialCollisionCheck(): void
	{
		// 计算下一帧的预期移动距离
		let moveDirection: number = this.BulletSpeed * Time.fixedDeltaTime;

		let hit: Ray = new Ray(this.gameObject.transform.position, this.gameObject.transform.up);
		let rifleRayHit: RaycastHit | undefined = Physics.Raycast(
			hit,
			moveDirection,
			-1,
			1
		);

		if (rifleRayHit != undefined)
		{
			let hitObj = rifleRayHit.collider.gameObject;

			if (hitObj.layer == this.BulletEnemyLayer)
			{
				let health: CharacterHealth | null = hitObj.GetComponent<CharacterHealth>(CharacterHealth);
				if (health != null)
				{
					health.TakeDamage(this.BulletDamage);
				}
			}

			this.gameObject.enable = false;
		}
	}

	private BulletFlyTimer(): boolean
	{
		if(this._currentFlyTimer >= this._bulletFlyTimer)
		{
			this._currentFlyTimer = 0;
			return true;
		}
		else
		{
			this._currentFlyTimer += Time.deltaTime;
		}

		return false;
	}
}
