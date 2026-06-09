class PlayerAnimation extends AnimationControllerBase
{
	// -- 玩家管理器 --
	public PlayerManager!: PlayerManager;

	// -- 动画剪辑 --
	public IdleClip!: AnimationClip;
	public MoveClip!: AnimationClip;
	public BackClip!: AnimationClip;
	public DeadClip!: AnimationClip;

	// -- 动画播放设置 --
	private _animPlayLoop: AnimationPlaySetting = new AnimationPlaySetting;
	private _animPlayOnce: AnimationPlaySetting = new AnimationPlaySetting;

	public OnStart(): void
	{
		// 设置动画播放设置
		this._animPlayLoop.wrapMode = WrapMode.Repeat;
		this._animPlayOnce.wrapMode = WrapMode.Clamp;

		// 添加动画剪辑
		this.AddAnimClip(this.IdleClip, "Idle", this._animPlayLoop);
		this.AddAnimClip(this.MoveClip, "Move", this._animPlayLoop);
		this.AddAnimClip(this.BackClip, "Back", this._animPlayLoop);
		this.AddAnimClip(this.DeadClip, "Dead", this._animPlayOnce);

		// 播放默认动画剪辑
		this.PlayAnim("Idle");
	}

	public OnUpdate(): void
	{
		this.CharacterAnimationController();
	}

	/**
	 * 玩家动画控制
	 */
	private CharacterAnimationController(): void
	{
		if (this.PlayerManager.IsDead)
			this.PlayAnim("Dead");
		else if (this.PlayerManager.MoveDirection.EqualsTo(Vector3.zero))
			this.PlayAnim("Idle");
		else
		{
			let dot: number = Vector3.Dot(this.PlayerManager.MoveDirection, this.PlayerManager.ViewDirection);
			if (dot >= 0)
				this.PlayAnim("Move");
			else
				this.PlayAnim("Back");
		}
	}
}
