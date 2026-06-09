class CharacterProgress extends CharacterProgressBase
{
	// ---- 进度数据来源 ----
	@EditorComponentSettings.DecorateName("生命值数据源")
	public HealthComponent!: CharacterHealth;		// 生命值数据源

	// ---- 进度配置 ----
	@EditorComponentSettings.DecorateName("血条UI组")
	public HealthControlName!: string;				// 血条UI名
	@EditorComponentSettings.DecorateName("当前血量UI组")
	public CurrentHealthControlName!: string;		// 当前血量UI名

	@EditorComponentSettings.DecorateName("当前血量UI组")
	private _currentHealthControl!: Image;			// 当前血量UI

	protected ContinueStart(): void
	{
		const healthControl = this._progressGroup.FindChild<Image>(Image, this.HealthControlName);
		if (healthControl == null)
		{
			Debug.Warning("CharacterProgress: HealthControl is null.");
			return;
		}

		const currentHealthControl = healthControl.FindChild<Image>(Image, this.CurrentHealthControlName);
		if (currentHealthControl == null)
		{
			Debug.Warning("CharacterProgress: CurrentHealthControl is null.");
			return;
		}

		this._currentHealthControl = currentHealthControl;
	}

	protected ContinueUpdate(): void
	{
		// 更新血量
		if(this.HealthComponent != null)
			this.SetHealth(this.HealthComponent.CurrentHealthPercentage);
	}

	/**
	 * 设置血量
	 * @param health 
	 */
	public SetHealth(health: number): void
	{
		this._currentHealthControl.fillAmount = health;
	}
}
