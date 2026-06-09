class PlayerProgress extends CharacterProgressBase
{
	// ---- 进度数据来源 ----
	@EditorComponentSettings.DecorateName("生命值数据源")
	public HealthComponent!: CharacterHealth;		// 生命值数据源
	@EditorComponentSettings.DecorateName("弹药数据源")
	public MagazineComponent!: RifleManager;			// 弹药数据源

	// ---- 进度配置 ----
	@EditorComponentSettings.DecorateName("血条UI名")
	public HealthControlName!: string;				// 血条UI名
	@EditorComponentSettings.DecorateName("当前血量UI名")
	public CurrentHealthControlName!: string;		// 当前血量UI名
	@EditorComponentSettings.DecorateName("弹夹容量UI名")
	public MagazineCapacityControlName!: string;		// 弹夹容量UI名
	@EditorComponentSettings.DecorateName("当前弹夹容量UI名")
	public CurrentMagazineControlName!: string;		// 当前弹夹容量UI名
	@EditorComponentSettings.DecorateName("装弹时间UI名")
	public ReloadTimeControlName!: string;			// 装弹时间UI名

	private _currentHealthControl!: Image;			// 当前血量UI
	private _currentMagazineControl!: Image;			// 当前弹夹容量UI
	private _reloadTimeControl!: Image;				// 装弹时间UI

	protected ContinueStart(): void
	{
		const healthControl = this._progressGroup.FindChild<Image>(Image, this.HealthControlName);
		const magazineCapacityControl = this._progressGroup.FindChild<Image>(Image, this.MagazineCapacityControlName);
		if (healthControl == null || magazineCapacityControl == null)
		{
			Debug.Warning("PlayerProgress: progress control is null.");
			return;
		}

		const currentHealthControl = healthControl.FindChild<Image>(Image, this.CurrentHealthControlName);
		const currentMagazineControl = magazineCapacityControl.FindChild<Image>(Image, this.CurrentMagazineControlName);
		const reloadTimeControl = magazineCapacityControl.FindChild<Image>(Image, this.ReloadTimeControlName);
		if (currentHealthControl == null || currentMagazineControl == null || reloadTimeControl == null)
		{
			Debug.Warning("PlayerProgress: child progress control is null.");
			return;
		}

		this._currentHealthControl = currentHealthControl;
		this._currentMagazineControl = currentMagazineControl;
		this._reloadTimeControl = reloadTimeControl;
	}

	protected ContinueUpdate(): void
	{
		// 更新血量
		if (this.HealthComponent != null)
			this.SetHealth(this.HealthComponent.CurrentHealthPercentage);

		// 更新弹药
		if (this.MagazineComponent != null)
			this.SetMagazineCapacity(this.MagazineComponent.CurrentMagazine / this.MagazineComponent.MagazineCapacity);

		// 更新装弹时间
		if (this.MagazineComponent != null && this.MagazineComponent.IsReloading)
		{
			this.SetMagazineReload(this.MagazineComponent.CurrentReloadTime / this.MagazineComponent.RifleReloadTime);
		}
	}

	public SetHealth(health: number): void
	{
		this._currentHealthControl.fillAmount = health;
	}

	public SetMagazineCapacity(magazineCapacity: number): void
	{
		this._currentMagazineControl.fillAmount = magazineCapacity;
	}

	public SetMagazineReload(reloadTime: number): void
	{
		this._reloadTimeControl.fillAmount = 1 - reloadTime;
		if (this._reloadTimeControl.fillAmount == 0)
		{
			// 填满弹药
			this.SetMagazineCapacity(1);
			this._currentHealthControl.fillAmount = 1;
		}
	}
}
