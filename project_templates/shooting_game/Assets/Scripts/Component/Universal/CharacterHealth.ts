class CharacterHealth extends Component
{
	// ---- 公共属性 ----
	@EditorComponentSettings.DecorateName("最大生命值")
	public HealthMax: number = 100;					// 最大生命值
	@EditorComponentSettings.DecorateName("是否自动恢复生命值")
	public IsAutoRegainHealth: boolean = false;		// 是否自动恢复生命值
	@EditorComponentSettings.DecorateName("自动恢复生命值速度")
	public AutoHealthRegainSpeed: number = 1;		// 恢复生命速度
	@EditorComponentSettings.DecorateName("自动恢复生命值间隔")
	public AutoHealthRegainInterval: number = 1;	// 恢复生命间隔

	// ---- 私有属性 ----
	private _currentHealth: number = 0;				// 目前生命值
	private _autoRegainHealthTimer: number = 0;		// 自动恢复生命值计时器
	private _isDead: boolean = false;				// 当前对象是否死亡

	// ---- Getter & Setter 访问器 ----
	public get IsDead(): boolean { return this._isDead; }
	public get CurrentHealth(): number { return this._currentHealth; }
	public get CurrentHealthPercentage(): number { return this._currentHealth / this.HealthMax; }

	public OnStart(): void
	{
		this._currentHealth = this.HealthMax;
		this._isDead = false;
		this._autoRegainHealthTimer = 0;
	}

	public OnUpdate(): void
	{
		// 自动恢复生命
		if (this.IsAutoRegainHealth)
			this.AutoRegainHealth();
	}

	/**
	 * 造成伤害
	 * @param damage 伤害量
	 */
	public TakeDamage(damage: number): void
	{
		if (this._isDead) return;

		this._currentHealth -= damage;

		// 重置自动恢复生命值计时器
		if (this.IsAutoRegainHealth)
			this._autoRegainHealthTimer = 0;

		// 当生命值低于 0 时，将生命值设为 0，并标记死亡
		if (this._currentHealth <= 0)
		{
			this._currentHealth = 0;
			this._isDead = true;
		}
	}

	/**
	 * 恢复生命值
	 * @param health 
	 */
	public RegainHealth(health: number): void
	{
		if (this._isDead) return;

		this._currentHealth += health;

		// 当生命值高于最大生命值时，将生命值设为最大生命值
		if (this._currentHealth > this.HealthMax)
			this._currentHealth = this.HealthMax;
	}

	/**
	 * 重置生命值和当前存活状态
	 */
	public RefresHealth(): void
	{
		this._currentHealth = this.HealthMax;
		this._isDead = false;
	}

	/**
	 * 自动恢复生命
	 * 当在 AutoHealthRegainInterval 时间内未受到伤害时
	 * 每过 AutoHealthRegainInterval 时间恢复 AutoHealthRegainSpeed 生命值
	 */
	private AutoRegainHealth(): void
	{
		// 若单位未死亡
		if (this._isDead == false)
		{
			// 检查计时器
			if (this._autoRegainHealthTimer <= this.AutoHealthRegainInterval)
			{
				this._autoRegainHealthTimer += Time.deltaTime;
			}
			else if (this._currentHealth <= this.HealthMax)
			{
				this.RegainHealth(this.AutoHealthRegainSpeed * Time.deltaTime);
			}
		}
	}
}