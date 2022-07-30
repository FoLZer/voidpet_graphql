use graphql_client::*;
use uuid::Uuid;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/AcceptQuest.graphql"
)]
pub struct AcceptQuest;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ActivateSpecialItem.graphql"
)]
pub struct ActivateSpecialItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/AdminGiveItems.graphql"
)]
pub struct AdminGiveItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/AdminGivePet.graphql"
)]
pub struct AdminGivePet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/AdminLoginAs.graphql"
)]
pub struct AdminLoginAs;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/AdminSendCustomNotif.graphql"
)]
pub struct AdminSendCustomNotif;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/AnswerGuildInvite.graphql"
)]
pub struct AnswerGuildInvite;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/AnswerTradeOffer.graphql"
)]
pub struct AnswerTradeOffer;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/AwardTrophy.graphql"
)]
pub struct AwardTrophy;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/BidOnAuctionItem.graphql"
)]
pub struct BidOnAuctionItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/BuyGigaItem.graphql"
)]
pub struct BuyGigaItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/BuyGigaMatterCheckoutLink.graphql"
)]
pub struct BuyGigaMatterCheckoutLink;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/BuyVendorItem.graphql"
)]
pub struct BuyVendorItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/BuyoutAuctionItem.graphql"
)]
pub struct BuyoutAuctionItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/CancelAuctionItem.graphql"
)]
pub struct CancelAuctionItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/CancelTrade.graphql"
)]
pub struct CancelTrade;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ClaimGigaMatter.graphql"
)]
pub struct ClaimGigaMatter;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/CreateAvatar.graphql"
)]
pub struct CreateAvatar;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/CreateStarter.graphql"
)]
pub struct CreateStarter;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/CreateTrade.graphql"
)]
pub struct CreateTrade;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/CreateTradeOffer.graphql"
)]
pub struct CreateTradeOffer;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/CreateTrophy.graphql"
)]
pub struct CreateTrophy;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DeleteGuild.graphql"
)]
pub struct DeleteGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DeleteMe.graphql"
)]
pub struct DeleteMe;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DeleteTrophy.graphql"
)]
pub struct DeleteTrophy;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DescendCave.graphql"
)]
pub struct DescendCave;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DevRandomItem.graphql"
)]
pub struct DevRandomItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DevSavePetItemPos.graphql"
)]
pub struct DevSavePetItemPos;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DrinkPotion.graphql"
)]
pub struct DrinkPotion;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/EatFoodItem.graphql"
)]
pub struct EatFoodItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/EditGuild.graphql"
)]
pub struct EditGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/EvolvePet.graphql"
)]
pub struct EvolvePet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/FleeBattle.graphql"
)]
pub struct FleeBattle;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ForceCatchPet.graphql"
)]
pub struct ForceCatchPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ForgetManyPets.graphql"
)]
pub struct ForgetManyPets;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ForgetPet.graphql"
)]
pub struct ForgetPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/FormGuild.graphql"
)]
pub struct FormGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GenDevPet.graphql"
)]
pub struct GenDevPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GigaCheckoutLink.graphql"
)]
pub struct GigaCheckoutLink;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GigaPortalLink.graphql"
)]
pub struct GigaPortalLink;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GiveUpCatching.graphql"
)]
pub struct GiveUpCatching;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/HealPets.graphql"
)]
pub struct HealPets;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/InviteToGuild.graphql"
)]
pub struct InviteToGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/LeaveGuild.graphql"
)]
pub struct LeaveGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/LinkClick.graphql"
)]
pub struct LinkClick;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/Logout.graphql"
)]
pub struct Logout;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/NameCaughtPet.graphql"
)]
pub struct NameCaughtPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/PetStorageCheckoutLink.graphql"
)]
pub struct PetStorageCheckoutLink;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/PickUpQuestItem.graphql"
)]
pub struct PickUpQuestItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/PlayWithPet.graphql"
)]
pub struct PlayWithPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/QuestInteraction.graphql"
)]
pub struct QuestInteraction;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ReportUser.graphql"
)]
pub struct ReportUser;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/SavePetMoves.graphql"
)]
pub struct SavePetMoves;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ScareAwayPet.graphql"
)]
pub struct ScareAwayPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/SellAuctionItem.graphql"
)]
pub struct SellAuctionItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/SendWildBattleSequence.graphql"
)]
pub struct SendWildBattleSequence;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/SetGuildRole.graphql"
)]
pub struct SetGuildRole;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/StabilizePlasma.graphql"
)]
pub struct StabilizePlasma;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/StartCatching.graphql"
)]
pub struct StartCatching;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/StartGuildBattle.graphql"
)]
pub struct StartGuildBattle;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/StartNpcBattle.graphql"
)]
pub struct StartNpcBattle;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/StartWildBattle.graphql"
)]
pub struct StartWildBattle;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/SwapDeadPet.graphql"
)]
pub struct SwapDeadPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/TakeIceCream.graphql"
)]
pub struct TakeIceCream;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/Teleport.graphql"
)]
pub struct Teleport;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ThrowItemAtPet.graphql"
)]
pub struct ThrowItemAtPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/TryToFriendCatching.graphql"
)]
pub struct TryToFriendCatching;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/TurnInQuest.graphql"
)]
pub struct TurnInQuest;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/UpdateAvatar.graphql"
)]
pub struct UpdateAvatar;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/UpdateMainPets.graphql"
)]
pub struct UpdateMainPets;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/UpdateMe.graphql"
)]
pub struct UpdateMe;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/UpdateMyLocation.graphql"
)]
pub struct UpdateMyLocation;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/UpdatePet.graphql"
)]
pub struct UpdatePet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/UpdatePetItems.graphql"
)]
pub struct UpdatePetItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/UpdateRep.graphql"
)]
pub struct UpdateRep;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/UsernameChangeCheckoutLink.graphql"
)]
pub struct UsernameChangeCheckoutLink;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DevAllItems.graphql"
)]
pub struct DevAllItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DevGetPetItemMap.graphql"
)]
pub struct DevGetPetItemMap;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DevPets.graphql"
)]
pub struct DevPets;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetAuctionItems.graphql"
)]
pub struct GetAuctionItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetAvatarItems.graphql"
)]
pub struct GetAvatarItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetBondLeaderboard.graphql"
)]
pub struct GetBondLeaderboard;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetDailyQuests.graphql"
)]
pub struct GetDailyQuests;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetGigaStoreItems.graphql"
)]
pub struct GetGigaStoreItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetGuild.graphql"
)]
pub struct GetGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetGuildInvites.graphql"
)]
pub struct GetGuildInvites;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetInventory.graphql"
)]
pub struct GetInventory;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetLinkAnalytics.graphql"
)]
pub struct GetLinkAnalytics;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetMyLocation.graphql"
)]
pub struct GetMyLocation;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetNotifs.graphql"
)]
pub struct GetNotifs;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetNumUnreadNotifs.graphql"
)]
pub struct GetNumUnreadNotifs;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetPet.graphql"
)]
pub struct GetPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetPetItems.graphql"
)]
pub struct GetPetItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetPetLog.graphql"
)]
pub struct GetPetLog;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetPetMoves.graphql"
)]
pub struct GetPetMoves;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetPets.graphql"
)]
pub struct GetPets;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetStarterAvatarItems.graphql"
)]
pub struct GetStarterAvatarItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetTradeBoxes.graphql"
)]
pub struct GetTradeBoxes;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetTradeOffers.graphql"
)]
pub struct GetTradeOffers;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetTrophies.graphql"
)]
pub struct GetTrophies;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetUser.graphql"
)]
pub struct GetUser;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetVendorItems.graphql"
)]
pub struct GetVendorItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ItemNameSearch.graphql"
)]
pub struct ItemNameSearch;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/Me2.graphql"
)]
pub struct Me2;