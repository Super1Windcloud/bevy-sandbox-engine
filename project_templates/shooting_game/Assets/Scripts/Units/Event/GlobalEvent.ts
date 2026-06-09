/**
 * 该类代表一个扩展了EventManager类的全局事件管理器。
 * 它是一个单例类，可以通过静态Instance属性访问。
 * @extends EventManager
 */
class GlobalEvent extends EventManager
{
	private static _instance: GlobalEvent | null = null;

	/**
	 * 返回 GlobalEvent 类的实例。
	 * 如果该实例不存在，则会创建一个新实例。
	 * @returns GlobalEvent 类的实例。
	 */
	public static get Instance(): GlobalEvent
	{
		if (!this._instance)
			this._instance = new GlobalEvent();
		return this._instance!;
	}

	private constructor()
	{
		super();
	}
}