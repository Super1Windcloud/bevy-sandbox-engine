class ZombieAnimation extends AnimationControllerBase
{
	// -- 动画剪辑 --
	public IdleClip!: AnimationClip;
	public MoveClip!: AnimationClip;
	public AttackClip!: AnimationClip;
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
		this.AddAnimClip(this.AttackClip, "Attack", this._animPlayLoop);
		this.AddAnimClip(this.DeadClip, "Dead", this._animPlayOnce);

		// 播放默认动画剪辑
		this.PlayAnim("Idle");
	}
}
