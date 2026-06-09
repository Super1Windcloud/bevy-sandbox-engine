abstract class AnimationControllerBase extends Component
{
	@EditorComponentSettings.DecorateName("动画组件所在对象")
	public AnimationModel!: Animation;

	// -- 动画名称数组 --
	private _arrayAnim: Array<string> = new Array<string>();

	private _currentAnimName: string = "";

	/**
	 * 播放动画剪辑
	 * @param AnimName
	 * @param transitionTime 
	 */
	public PlayAnim(AnimName: string, transitionTime: number = 0.1, isPlay?: boolean): void
	{
		// 检查动画名称是否存在
		if (this._arrayAnim.indexOf(AnimName) == -1)
		{
			Debug.Warning("Animation name is not exist: " + AnimName);
			return;
		}

		// 如果动画已经在播放，请勿再次播放
		if (this.AnimationModel.IsPlaying(AnimName))
		{
			return;
		}

		if (this._currentAnimName == AnimName)
		{
			return;
		}
		else
		{
			this._currentAnimName = AnimName;
		}

		if (isPlay == true)
			this.AnimationModel.Play(AnimName, AnimationPlayMode.StopAll);
		else
			this.AnimationModel.CrossFade(AnimName, transitionTime);
	}

	/**
	 * 将指定的动画片段添加到指定名称的动画组件中
	 * 并将其添加到动画名称数组中
	 * @param clip 动画剪辑
	 * @param name 动画名称
	 * @param playSetting 播放设置
	 */
	protected AddAnimClip(clip: AnimationClip, name: string, playSetting?: AnimationPlaySetting): void
	{
		if (this._arrayAnim.indexOf(name) != -1)
		{
			Debug.Warning("动画名称重复: " + name);
			return;
		}
		if (playSetting == null)
			this.AnimationModel.AddClip(clip, name);
		else
			this.AnimationModel.AddClip(clip, name, playSetting);

		this._arrayAnim.push(name);
	}
}
