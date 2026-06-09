class RifleAttributes extends Component
{
	// ---- 步枪效果和音频 ----
	@EditorComponentSettings.DecorateName("步枪射击特效")
	public RifleShootEffect!: ParticleSystem;
	@EditorComponentSettings.DecorateName("步枪射击音效")
	public RifleShootAudio!: AudioSource;

	// ---- getter && setter ----
	public get GetRifleShootEffect(): ParticleSystem { return this.RifleShootEffect; }
	public get GetRifleShootAudio(): AudioSource { return this.RifleShootAudio; }
}
