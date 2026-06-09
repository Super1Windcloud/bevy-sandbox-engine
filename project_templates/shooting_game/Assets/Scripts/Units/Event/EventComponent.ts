@ComponentSettings.ExecuteOrder(100)
class EventComponent extends Component
{
	private _localEvent: EventManager | null = null;

	public OnEnable(): void
	{
		this._localEvent = new EventManager();
	}

	public get LocalEvent(): EventManager
	{
		return this._localEvent!;
	}
}